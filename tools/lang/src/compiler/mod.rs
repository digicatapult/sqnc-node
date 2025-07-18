use serde::Serialize;
use sqnc_runtime_types::{
    ArgType, BooleanExpressionSymbol, BooleanOperator, ProcessIdentifier, ProcessVersion, RuntimeExpressionSymbol,
    RuntimeProgram, TokenMetadataKey, TokenMetadataValue,
};
use std::collections::HashMap;

use crate::{
    ast::{
        types::{AstNode, FnArg, FnDecl, TokenDecl, TokenFieldType},
        Ast,
    },
    errors::{CompilationError, CompilationStage, ErrorVariant, PestError},
};

mod constants;

mod flatten;
pub use flatten::flatten_fns;

mod token_transform;
pub use token_transform::token_decl_to_conditions;

mod condition_transform;
pub use condition_transform::transform_condition_to_program;

mod helper;
use helper::to_bounded_vec;

use self::constants::{TYPE_KEY, VERSION_KEY};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArgumentField {
    pub(crate) name: String,
    pub(crate) allow_token: bool,
    pub(crate) allow_role: bool,
    pub(crate) allow_file: bool,
    pub(crate) allow_literal: bool,
    pub(crate) allow_none: bool,
    pub(crate) allowed_literal_values: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputArgument {
    pub(crate) name: String,
    pub(crate) is_reference: bool,
    pub(crate) token_type: String,
    pub(crate) version: u32,
    pub(crate) fields: Vec<ArgumentField>,
}

#[derive(Serialize)]
pub struct OutputArgument {
    pub(crate) name: String,
    pub(crate) token_type: String,
    pub(crate) version: u32,
    pub(crate) fields: Vec<ArgumentField>,
}

#[derive(Serialize)]
pub struct Arguments {
    pub(crate) inputs: Vec<InputArgument>,
    pub(crate) outputs: Vec<OutputArgument>,
}

#[derive(Serialize)]
pub struct Process {
    pub(crate) name: ProcessIdentifier,
    pub(crate) version: ProcessVersion,
    pub(crate) program: RuntimeProgram,
    pub(crate) arguments: Arguments,
}

fn make_arg_type_restrictions<'a, I>(
    arg_type: ArgType,
    token_decls: &HashMap<&str, TokenDecl>,
    iter: I,
) -> Result<Vec<Vec<RuntimeExpressionSymbol>>, CompilationError>
where
    I: Iterator<Item = &'a AstNode<'a, FnArg<'a>>>,
{
    let token_type_key = TokenMetadataKey::try_from(TYPE_KEY.to_vec()).unwrap();
    let version_type_key = TokenMetadataKey::try_from(VERSION_KEY.to_vec()).unwrap();

    iter.enumerate()
        .map(|(index, arg)| {
            let token_decl = token_decls.get(arg.value.token_type.value).ok_or(CompilationError {
                stage: CompilationStage::ReduceTokens,
                exit_code: exitcode::DATAERR,
                inner: PestError::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!("Unknown token type {}", arg.value.token_type.value),
                    },
                    arg.value.token_type.span,
                ),
            })?;
            let version = token_decl.attributes.value.version;
            let version = version.to_string();

            Ok(vec![
                BooleanExpressionSymbol::Restriction(sqnc_runtime_types::Restriction::FixedArgMetadataValue {
                    arg_type,
                    index: index as u32,
                    metadata_key: token_type_key.clone(),
                    metadata_value: TokenMetadataValue::Literal(to_bounded_vec(AstNode {
                        value: arg.value.token_type.value.as_bytes().to_owned(),
                        span: arg.value.token_type.span,
                    })?),
                }),
                BooleanExpressionSymbol::Op(BooleanOperator::And),
                BooleanExpressionSymbol::Restriction(sqnc_runtime_types::Restriction::FixedArgMetadataValue {
                    arg_type,
                    index: index as u32,
                    metadata_key: version_type_key.clone(),
                    metadata_value: TokenMetadataValue::Literal(to_bounded_vec(AstNode {
                        value: version.as_bytes().to_owned(),
                        span: arg.value.token_type.span,
                    })?),
                }),
            ])
        })
        .collect::<Result<Vec<_>, _>>()
}

fn make_process_restrictions(
    fn_decl: &FnDecl,
    token_decls: &HashMap<&str, TokenDecl>,
) -> Result<RuntimeProgram, CompilationError> {
    // chain inputs to outputs, transform each to conditions, flatten then chain on the fn conditions
    let conditions = fn_decl
        .inputs
        .value
        .iter()
        .chain(fn_decl.outputs.value.iter())
        .map(|arg| {
            let token_decl = token_decls.get(arg.value.token_type.value).ok_or(CompilationError {
                stage: CompilationStage::ReduceTokens,
                exit_code: exitcode::DATAERR,
                inner: PestError::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!("Unknown token type {}", arg.value.token_type.value),
                    },
                    arg.value.token_type.span,
                ),
            })?;

            token_decl_to_conditions(arg.value.name.clone(), token_decl)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .chain(fn_decl.conditions.clone().value.into_iter())
        .collect::<Vec<_>>();

    // get restrictions for different arg types
    let (input_refs, input_tokens): (Vec<_>, Vec<_>) = fn_decl
        .inputs
        .value
        .into_iter()
        .partition(|input| input.value.is_reference);

    let input_ref_count = input_refs.len();
    let input_token_count = input_tokens.len();
    let output_count = fn_decl.outputs.value.len();

    let input_ref_conditions = make_arg_type_restrictions(ArgType::Reference, token_decls, input_refs.into_iter())?;
    let input_token_conditions = make_arg_type_restrictions(ArgType::Input, token_decls, input_tokens.into_iter())?;
    let output_conditions =
        make_arg_type_restrictions(ArgType::Output, token_decls, fn_decl.outputs.value.into_iter())?;

    // loop through conditions to build program
    let condition_programs = conditions
        .into_iter()
        .map(|condition| transform_condition_to_program(&fn_decl, token_decls, condition))
        .collect::<Result<Vec<_>, _>>()?;

    let program: Vec<_> = [
        vec![BooleanExpressionSymbol::Restriction(
            sqnc_runtime_types::Restriction::FixedArgCount {
                arg_type: ArgType::Reference,
                count: input_ref_count as u32,
            },
        )],
        vec![BooleanExpressionSymbol::Restriction(
            sqnc_runtime_types::Restriction::FixedArgCount {
                arg_type: ArgType::Input,
                count: input_token_count as u32,
            },
        )],
        vec![BooleanExpressionSymbol::Restriction(
            sqnc_runtime_types::Restriction::FixedArgCount {
                arg_type: ArgType::Output,
                count: output_count as u32,
            },
        )],
    ]
    .into_iter()
    .chain(input_ref_conditions)
    .chain(input_token_conditions)
    .chain(output_conditions)
    .chain(condition_programs)
    .enumerate()
    .map(|(index, mut expression)| match index {
        0 => expression,
        _ => {
            expression.push(BooleanExpressionSymbol::Op(BooleanOperator::And));
            expression
        }
    })
    .flatten()
    .collect();

    to_bounded_vec(AstNode {
        value: program,
        span: fn_decl.conditions.span,
    })
}

fn map_token_props_to_arguments(token_decl: &TokenDecl) -> Vec<ArgumentField> {
    token_decl
        .props
        .value
        .iter()
        .map(|prop| ArgumentField {
            name: prop.value.name.value.to_owned(),
            allow_token: prop
                .value
                .types
                .iter()
                .find(|t| matches!(t.value, TokenFieldType::Token(_)))
                .is_some(),
            allow_role: prop
                .value
                .types
                .iter()
                .find(|t| matches!(t.value, TokenFieldType::Role))
                .is_some(),
            allow_file: prop
                .value
                .types
                .iter()
                .find(|t| matches!(t.value, TokenFieldType::File))
                .is_some(),
            allow_literal: prop
                .value
                .types
                .iter()
                .find(|t| matches!(t.value, TokenFieldType::Literal))
                .is_some(),
            allow_none: prop
                .value
                .types
                .iter()
                .find(|t| matches!(t.value, TokenFieldType::None))
                .is_some(),
            allowed_literal_values: prop
                .value
                .types
                .iter()
                .filter_map(|t| match &t.value {
                    TokenFieldType::LiteralValue(v) => Some(v.value.to_owned()),
                    _ => None,
                })
                .collect(),
        })
        .collect()
}

fn make_process_arguments(fn_decl: &FnDecl, token_decls: &HashMap<&str, TokenDecl>) -> Arguments {
    Arguments {
        inputs: fn_decl
            .inputs
            .value
            .iter()
            .map(|input| {
                let decl = token_decls.get(input.value.token_type.value).unwrap();
                InputArgument {
                    name: input.value.name.value.to_owned(),
                    token_type: decl.name.value.to_owned(),
                    version: decl.attributes.value.version,
                    fields: map_token_props_to_arguments(token_decls.get(input.value.token_type.value).unwrap()),
                    is_reference: input.value.is_reference,
                }
            })
            .collect(),
        outputs: fn_decl
            .outputs
            .value
            .iter()
            .map(|output| {
                let decl = token_decls.get(output.value.token_type.value).unwrap();
                OutputArgument {
                    name: output.value.name.value.to_owned(),
                    token_type: decl.name.value.to_owned(),
                    version: decl.attributes.value.version,
                    fields: map_token_props_to_arguments(token_decls.get(output.value.token_type.value).unwrap()),
                }
            })
            .collect(),
    }
}

pub fn compile_ast_to_restrictions(ast: Ast) -> Result<Vec<Process>, CompilationError> {
    let ast = flatten_fns(ast)?;

    let (token_nodes, fn_nodes): (Vec<_>, Vec<_>) = ast.into_iter().partition(|node| match node.value {
        crate::ast::types::AstRoot::TokenDecl(_) => true,
        crate::ast::types::AstRoot::FnDecl(_) => false,
    });

    let token_decls = token_nodes
        .into_iter()
        .map(|node| match node.value {
            crate::ast::types::AstRoot::TokenDecl(decl) => (decl.value.name.value, decl.value),
            crate::ast::types::AstRoot::FnDecl(_) => panic!(),
        })
        .collect::<HashMap<_, _>>();

    fn_nodes
        .into_iter()
        .map(move |node| match node.value {
            crate::ast::types::AstRoot::TokenDecl(_) => panic!(),
            crate::ast::types::AstRoot::FnDecl(f) => {
                let f = f.value;
                Ok(Process {
                    name: to_bounded_vec(AstNode {
                        value: f.name.value.as_bytes().to_vec(),
                        span: f.name.span,
                    })?,
                    version: f.attributes.value.version,
                    program: make_process_restrictions(&f, &token_decls)?,
                    arguments: make_process_arguments(&f, &token_decls),
                })
            }
        })
        .collect::<Result<Vec<_>, _>>()
}
