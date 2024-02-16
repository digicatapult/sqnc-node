use serde::Serialize;
use sqnc_runtime_types::{
    BooleanExpressionSymbol, BooleanOperator, ProcessIdentifier, ProcessVersion, RuntimeProgram, TokenMetadataKey,
    TokenMetadataValue,
};
use std::collections::HashMap;

use crate::{
    ast::{
        types::{AstNode, FnDecl, TokenDecl},
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

    // get restrictions for input arg types
    let token_type_key = TokenMetadataKey::try_from(TYPE_KEY.to_vec()).unwrap();
    let version_type_key = TokenMetadataKey::try_from(VERSION_KEY.to_vec()).unwrap();
    let input_arg_conditions = fn_decl
        .inputs
        .value
        .iter()
        .enumerate()
        .map(|(index, input)| {
            Ok(vec![
                BooleanExpressionSymbol::Restriction(sqnc_runtime_types::Restriction::FixedInputMetadataValue {
                    index: index as u32,
                    metadata_key: token_type_key.clone(),
                    metadata_value: TokenMetadataValue::Literal(to_bounded_vec(AstNode {
                        value: input.value.token_type.value.as_bytes().to_owned(),
                        span: input.value.token_type.span,
                    })?),
                }),
                BooleanExpressionSymbol::Op(BooleanOperator::And),
                BooleanExpressionSymbol::Restriction(sqnc_runtime_types::Restriction::FixedInputMetadataValue {
                    index: index as u32,
                    metadata_key: version_type_key.clone(),
                    metadata_value: TokenMetadataValue::Literal(to_bounded_vec(AstNode {
                        value: "1".as_bytes().to_owned(),
                        span: input.value.token_type.span,
                    })?),
                }),
            ])
        })
        .collect::<Result<Vec<_>, _>>()?;

    // get restrictions for output arg types
    let output_arg_conditions = fn_decl
        .outputs
        .value
        .iter()
        .enumerate()
        .map(|(index, output)| {
            Ok(vec![
                BooleanExpressionSymbol::Restriction(sqnc_runtime_types::Restriction::FixedOutputMetadataValue {
                    index: index as u32,
                    metadata_key: token_type_key.clone(),
                    metadata_value: TokenMetadataValue::Literal(to_bounded_vec(AstNode {
                        value: output.value.token_type.value.as_bytes().to_owned(),
                        span: output.value.token_type.span,
                    })?),
                }),
                BooleanExpressionSymbol::Op(BooleanOperator::And),
                BooleanExpressionSymbol::Restriction(sqnc_runtime_types::Restriction::FixedOutputMetadataValue {
                    index: index as u32,
                    metadata_key: version_type_key.clone(),
                    metadata_value: TokenMetadataValue::Literal(to_bounded_vec(AstNode {
                        value: "1".as_bytes().to_owned(),
                        span: output.value.token_type.span,
                    })?),
                }),
            ])
        })
        .collect::<Result<Vec<_>, _>>()?;

    // loop through conditions to build program
    let condition_programs = conditions
        .into_iter()
        .map(|condition| transform_condition_to_program(&fn_decl, token_decls, condition))
        .collect::<Result<Vec<_>, _>>()?;

    let program: Vec<_> = [
        vec![BooleanExpressionSymbol::Restriction(
            sqnc_runtime_types::Restriction::FixedNumberOfInputs {
                num_inputs: fn_decl.inputs.value.len() as u32,
            },
        )],
        vec![BooleanExpressionSymbol::Restriction(
            sqnc_runtime_types::Restriction::FixedNumberOfOutputs {
                num_outputs: fn_decl.outputs.value.len() as u32,
            },
        )],
    ]
    .into_iter()
    .chain(input_arg_conditions)
    .chain(output_arg_conditions)
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
