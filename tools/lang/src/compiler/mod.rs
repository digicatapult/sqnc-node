use dscp_runtime_types::{
    BooleanExpressionSymbol, BooleanOperator, ProcessIdentifier, ProcessVersion, RuntimeProgram, TokenMetadataKey,
    TokenMetadataValue,
};
use serde::Serialize;

use std::{collections::HashMap, fmt::Display};

mod constants;

mod parse;
pub use parse::parse_str_to_ast;

mod flatten;
pub use flatten::flatten_fns;

mod token_transform;
pub use token_transform::token_decl_to_conditions;

mod condition_transform;
pub use condition_transform::transform_condition_to_program;

use crate::{
    ast::types::{AstNode, FnDecl, TokenDecl},
    errors::{CompilationError, ErrorVariant, PestError},
};

use self::constants::TYPE_KEY;

#[derive(Debug, PartialEq)]
pub enum CompilationStage {
    ParseGrammar,
    BuildAst,
    LengthValidation,
    ReduceFns,
    ReduceTokens,
    GenerateRestrictions,
}

impl Display for CompilationStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilationStage::ParseGrammar => write!(f, "parsing grammar"),
            CompilationStage::BuildAst => write!(f, "building ast"),
            CompilationStage::ReduceFns => write!(f, "parsing function definitions"),
            CompilationStage::ReduceTokens => write!(f, "reducing tokens to constraints"),
            CompilationStage::LengthValidation => write!(f, "validating length of output"),
            CompilationStage::GenerateRestrictions => write!(f, "generating restrictions"),
        }
    }
}

#[derive(Serialize)]
pub struct Process {
    pub(crate) name: ProcessIdentifier,
    pub(crate) version: ProcessVersion,
    pub(crate) program: RuntimeProgram,
}

fn make_process_restrictions(
    fn_decl: FnDecl,
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

    let input_arg_conditions = fn_decl
        .inputs
        .value
        .iter()
        .enumerate()
        .map(|(index, input)| {
            Ok(BooleanExpressionSymbol::Restriction(
                dscp_runtime_types::Restriction::FixedInputMetadataValue {
                    index: index as u32,
                    metadata_key: TokenMetadataKey::try_from(TYPE_KEY.to_vec()).unwrap(),
                    metadata_value: TokenMetadataValue::Literal(to_bounded_vec(AstNode {
                        value: input.value.token_type.value.as_bytes().to_owned(),
                        span: input.value.token_type.span,
                    })?),
                },
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let output_arg_conditions = fn_decl
        .outputs
        .value
        .iter()
        .enumerate()
        .map(|(index, output)| {
            Ok(BooleanExpressionSymbol::Restriction(
                dscp_runtime_types::Restriction::FixedOutputMetadataValue {
                    index: index as u32,
                    metadata_key: TokenMetadataKey::try_from(TYPE_KEY.to_vec()).unwrap(),
                    metadata_value: TokenMetadataValue::Literal(to_bounded_vec(AstNode {
                        value: output.value.token_type.value.as_bytes().to_owned(),
                        span: output.value.token_type.span,
                    })?),
                },
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // loop through conditions to build program
    let condition_programs = conditions
        .into_iter()
        .map(|condition| transform_condition_to_program(&fn_decl, token_decls, condition))
        .collect::<Result<Vec<_>, _>>()?;

    let combine_ops = vec![
        BooleanExpressionSymbol::Op(BooleanOperator::And);
        input_arg_conditions.len() + condition_programs.len() - 1
    ];
    let program: Vec<_> = input_arg_conditions
        .into_iter()
        .chain(output_arg_conditions)
        .chain(condition_programs.into_iter().flatten())
        .chain(combine_ops)
        .collect();

    to_bounded_vec(AstNode {
        value: program,
        span: fn_decl.conditions.span,
    })
}

pub fn to_bounded_vec<I, O, V>(collection: AstNode<I>) -> Result<O, CompilationError>
where
    I: IntoIterator<Item = V>,
    O: TryFrom<Vec<V>>,
{
    let foo = collection.value.into_iter().collect::<Vec<_>>();
    let foo_len = foo.len();
    <O as TryFrom<Vec<V>>>::try_from(foo).map_err(|_| CompilationError {
        stage: CompilationStage::LengthValidation,
        exit_code: exitcode::DATAERR,
        inner: PestError::new_from_span(
            pest::error::ErrorVariant::CustomError {
                message: format!("too long or compiles to too many elements ({})", foo_len),
            },
            collection.span,
        ),
    })
}

pub fn compile_input_to_restrictions(input: &str) -> Result<Vec<Process>, CompilationError> {
    let ast = parse_str_to_ast(input)?;
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
        .map(|node| match node.value {
            crate::ast::types::AstRoot::TokenDecl(_) => panic!(),
            crate::ast::types::AstRoot::FnDecl(f) => {
                let f = f.value;
                Ok(Process {
                    name: to_bounded_vec(AstNode {
                        value: f.name.value.as_bytes().to_vec(),
                        span: f.name.span,
                    })?,
                    version: 1u32,
                    program: make_process_restrictions(f, &token_decls)?,
                })
            }
        })
        .collect::<Result<Vec<_>, _>>()
}
