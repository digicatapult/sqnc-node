use std::{collections::HashMap, sync::Arc};

use sqnc_runtime_types::{
    ArgType, BooleanExpressionSymbol, BooleanOperator, MetadataValue, MetadataValueType, Restriction,
    RuntimeExpressionSymbol, TokenMetadataKey,
};

use crate::{
    ast::types::{
        AstNode, BoolCmp, BoolOp, Comparison, ExpressionTree, FnArg, FnDecl, TokenDecl, TokenFieldType, TokenProp,
        TypeCmp, TypeCmpType,
    },
    compiler::CompilationStage,
    errors::{CompilationError, ErrorVariant, PestError},
};

use super::{constants::ORIGINAL_ID_KEY, to_bounded_vec};

struct TokenLocation<'a> {
    is_input: bool,
    index: u32,
    arg: &'a FnArg<'a>,
}

struct TokenPropLocation<'a> {
    is_input: bool,
    index: u32,
    arg: &'a FnArg<'a>,
    prop: &'a str,
    types: Arc<[AstNode<'a, TokenFieldType<'a>>]>,
}

fn find_token<'a>(
    fn_decl: &'a FnDecl<'a>,
    name: &'a AstNode<'a, &'a str>,
) -> Result<TokenLocation<'a>, CompilationError> {
    let find_input = fn_decl
        .inputs
        .value
        .iter()
        .enumerate()
        .find(|(.., input)| input.value.name.value == name.value);

    let find_output = fn_decl
        .outputs
        .value
        .iter()
        .enumerate()
        .find(|(.., output)| output.value.name.value == name.value);

    let (is_input, index, arg) = match (find_input, find_output) {
        (None, None) => Err(CompilationError {
            stage: CompilationStage::GenerateRestrictions,
            exit_code: exitcode::DATAERR,
            inner: PestError::new_from_span(
                ErrorVariant::CustomError {
                    message: "Unknown token".into(),
                },
                name.span,
            ),
        }),
        (None, Some((index, arg))) => Ok((false, index, arg)),
        (Some((index, arg)), None) => Ok((true, index, arg)),
        (Some(_), Some(_)) => Err(CompilationError {
            stage: CompilationStage::GenerateRestrictions,
            exit_code: exitcode::SOFTWARE,
            inner: PestError::new_from_span(
                ErrorVariant::CustomError {
                    message: "Unexpected error. Function args should be unique by this point in compilation".into(),
                },
                name.span,
            ),
        }),
    }?;

    Ok(TokenLocation {
        is_input,
        index: index as u32,
        arg: &arg.value,
    })
}

fn find_token_prop<'a>(
    token_decls: &HashMap<&'a str, TokenDecl<'a>>,
    fn_decl: &'a FnDecl<'a>,
    prop: &'a TokenProp<'a>,
) -> Result<TokenPropLocation<'a>, CompilationError> {
    let TokenLocation { is_input, index, arg } = find_token(fn_decl, &prop.token)?;

    let token_decl = token_decls.get(arg.token_type.value).ok_or(CompilationError {
        stage: CompilationStage::GenerateRestrictions,
        exit_code: exitcode::DATAERR,
        inner: PestError::new_from_span(
            ErrorVariant::CustomError {
                message: format!("Unknown token type {}", arg.token_type.value),
            },
            arg.token_type.span,
        ),
    })?;

    let prop = token_decl
        .props
        .value
        .iter()
        .find(|prop_decl| prop_decl.value.name.value == prop.prop.value)
        .ok_or(CompilationError {
            stage: CompilationStage::GenerateRestrictions,
            exit_code: exitcode::DATAERR,
            inner: PestError::new_from_span(
                ErrorVariant::CustomError {
                    message: format!(
                        "Unknown property {} on token type {}",
                        prop.prop.value, arg.token_type.value
                    ),
                },
                prop.prop.span,
            ),
        })?;

    Ok(TokenPropLocation {
        index: index as u32,
        is_input,
        arg,
        prop: prop.value.name.value,
        types: prop.value.types.clone(),
    })
}

pub fn transform_condition_to_program(
    fn_decl: &FnDecl,
    token_decls: &HashMap<&str, TokenDecl>,
    expression: ExpressionTree,
) -> Result<Vec<RuntimeExpressionSymbol>, CompilationError> {
    match expression {
        ExpressionTree::Leaf(comp) => {
            let AstNode { value: comp, span } = comp;
            match comp {
                Comparison::Fn { .. } => Err(CompilationError {
                    stage: crate::compiler::CompilationStage::ReduceTokens,
                    exit_code: exitcode::SOFTWARE,
                    inner: PestError::new_from_span(
                        ErrorVariant::CustomError {
                            message: "Internal Error. Unexpected function call (should have been flattened)?".into(),
                        },
                        span,
                    ),
                }),
                Comparison::PropLit { left, op, right } => {
                    let TokenPropLocation {
                        is_input, index, types, ..
                    } = find_token_prop(token_decls, fn_decl, &left.value)?;
                    if types
                        .iter()
                        .find(|field_type| match &field_type.value {
                            TokenFieldType::Literal => true,
                            TokenFieldType::LiteralValue(v) => v.value == right.value,
                            _ => false,
                        })
                        .is_none()
                    {
                        return Err(CompilationError {
                            stage: crate::compiler::CompilationStage::GenerateRestrictions,
                            exit_code: exitcode::DATAERR,
                            inner: PestError::new_from_span(
                                ErrorVariant::CustomError {
                                    message: format!(
                                        "Invalid comparison between property {} and value {}",
                                        left.value.prop.value, right.value
                                    ),
                                },
                                span,
                            ),
                        });
                    }

                    let metadata_key = to_bounded_vec(AstNode {
                        value: left.value.prop.value.as_bytes().to_owned(),
                        span: left.value.prop.span,
                    })?;

                    let metadata_value = MetadataValue::Literal(to_bounded_vec(AstNode {
                        value: right.value.as_bytes().to_owned(),
                        span: right.span,
                    })?);

                    let mut result = vec![match is_input {
                        true => BooleanExpressionSymbol::Restriction(Restriction::FixedArgMetadataValue {
                            arg_type: ArgType::Input,
                            index,
                            metadata_key,
                            metadata_value,
                        }),
                        false => BooleanExpressionSymbol::Restriction(Restriction::FixedArgMetadataValue {
                            arg_type: ArgType::Output,
                            index,
                            metadata_key,
                            metadata_value,
                        }),
                    }];

                    if op == BoolCmp::Neq {
                        result.append(&mut vec![
                            BooleanExpressionSymbol::Restriction(Restriction::None),
                            BooleanExpressionSymbol::Op(BooleanOperator::NotL),
                        ]);
                    }

                    Ok(result)
                }
                Comparison::PropInt { left, op, right } => {
                    let TokenPropLocation {
                        is_input, index, types, ..
                    } = find_token_prop(token_decls, fn_decl, &left.value)?;
                    if types
                        .iter()
                        .find(|field_type| match &field_type.value {
                            TokenFieldType::Integer => true,
                            TokenFieldType::IntegerValue(v) => v.value == right.value,
                            _ => false,
                        })
                        .is_none()
                    {
                        return Err(CompilationError {
                            stage: crate::compiler::CompilationStage::GenerateRestrictions,
                            exit_code: exitcode::DATAERR,
                            inner: PestError::new_from_span(
                                ErrorVariant::CustomError {
                                    message: format!(
                                        "Invalid comparison between property {} and value {}",
                                        left.value.prop.value, right.value
                                    ),
                                },
                                span,
                            ),
                        });
                    }

                    let metadata_key = to_bounded_vec(AstNode {
                        value: left.value.prop.value.as_bytes().to_owned(),
                        span: left.value.prop.span,
                    })?;

                    let metadata_value = MetadataValue::Integer(right.value);

                    let mut result = vec![match is_input {
                        true => BooleanExpressionSymbol::Restriction(Restriction::FixedArgMetadataValue {
                            arg_type: ArgType::Input,
                            index,
                            metadata_key,
                            metadata_value,
                        }),
                        false => BooleanExpressionSymbol::Restriction(Restriction::FixedArgMetadataValue {
                            arg_type: ArgType::Output,
                            index,
                            metadata_key,
                            metadata_value,
                        }),
                    }];

                    if op == BoolCmp::Neq {
                        result.append(&mut vec![
                            BooleanExpressionSymbol::Restriction(Restriction::None),
                            BooleanExpressionSymbol::Op(BooleanOperator::NotL),
                        ]);
                    }

                    Ok(result)
                }
                Comparison::PropSender { left, op } => {
                    let TokenPropLocation {
                        is_input, index, types, ..
                    } = find_token_prop(token_decls, fn_decl, &left.value)?;
                    if types
                        .iter()
                        .find(|field_type| match &field_type.value {
                            TokenFieldType::Role => true,
                            _ => false,
                        })
                        .is_none()
                    {
                        return Err(CompilationError {
                            stage: crate::compiler::CompilationStage::GenerateRestrictions,
                            exit_code: exitcode::DATAERR,
                            inner: PestError::new_from_span(
                                ErrorVariant::CustomError {
                                    message: format!(
                                        "Cannot compare property {} to sender as it is not a Role",
                                        left.value.prop.value,
                                    ),
                                },
                                span,
                            ),
                        });
                    }

                    let role_key = to_bounded_vec(AstNode {
                        value: left.value.prop.value.as_bytes().to_owned(),
                        span: left.value.prop.span,
                    })?;

                    let mut result = vec![match is_input {
                        true => BooleanExpressionSymbol::Restriction(Restriction::SenderHasArgRole {
                            arg_type: ArgType::Input,
                            index,
                            role_key,
                        }),
                        false => BooleanExpressionSymbol::Restriction(Restriction::SenderHasArgRole {
                            arg_type: ArgType::Output,
                            index,
                            role_key,
                        }),
                    }];

                    if op == BoolCmp::Neq {
                        result.append(&mut vec![
                            BooleanExpressionSymbol::Restriction(Restriction::None),
                            BooleanExpressionSymbol::Op(BooleanOperator::NotL),
                        ]);
                    }

                    Ok(result)
                }
                Comparison::TokenToken { left, op, right } => {
                    let left = find_token(fn_decl, &left)?;
                    let right = find_token(fn_decl, &right)?;

                    let (input, output) = match (&left.is_input, &right.is_input) {
                        (true, false) => Ok((left, right)),
                        (false, true) => Ok((right, left)),
                        _ => Err(CompilationError {
                            stage: crate::compiler::CompilationStage::GenerateRestrictions,
                            exit_code: exitcode::DATAERR,
                            inner: PestError::new_from_span(
                                ErrorVariant::CustomError {
                                    message: "Token comparisons must be between an input and an output".into(),
                                },
                                span,
                            ),
                        }),
                    }?;

                    let original_key = TokenMetadataKey::try_from(ORIGINAL_ID_KEY.to_vec()).unwrap();
                    let result = vec![
                        BooleanExpressionSymbol::Restriction(Restriction::MatchArgsMetadataValue {
                            left_arg_type: ArgType::Input,
                            left_index: input.index,
                            left_metadata_key: original_key.clone(),
                            right_arg_type: ArgType::Output,
                            right_index: output.index,
                            right_metadata_key: original_key.clone(),
                        }),
                        BooleanExpressionSymbol::Restriction(Restriction::ArgHasMetadata {
                            arg_type: ArgType::Input,
                            index: input.index,
                            metadata_key: original_key.clone(),
                        }),
                        BooleanExpressionSymbol::Restriction(Restriction::MatchArgIdToMetadataValue {
                            left_arg_type: ArgType::Input,
                            left_index: input.index,
                            right_arg_type: ArgType::Output,
                            right_index: output.index,
                            right_metadata_key: original_key.clone(),
                        }),
                        BooleanExpressionSymbol::Op(BooleanOperator::InhibitionR),
                        BooleanExpressionSymbol::Op(match op == BoolCmp::Eq {
                            true => BooleanOperator::Xor,
                            false => BooleanOperator::Xnor,
                        }),
                    ];

                    Ok(result)
                }
                Comparison::PropToken { left, op, right } => {
                    let output = find_token_prop(token_decls, fn_decl, &left.value)?;

                    if output.is_input {
                        return Err(CompilationError {
                            stage: crate::compiler::CompilationStage::GenerateRestrictions,
                            exit_code: exitcode::DATAERR,
                            inner: PestError::new_from_span(
                                ErrorVariant::CustomError {
                                    message: "Cannot assert a property on an input equates to a token".into(),
                                },
                                span,
                            ),
                        });
                    }

                    let input = find_token(fn_decl, &right)?;

                    if !input.is_input {
                        return Err(CompilationError {
                            stage: crate::compiler::CompilationStage::GenerateRestrictions,
                            exit_code: exitcode::DATAERR,
                            inner: PestError::new_from_span(
                                ErrorVariant::CustomError {
                                    message: "Cannot assert a token property equates to an output token".into(),
                                },
                                span,
                            ),
                        });
                    }

                    if output
                        .types
                        .iter()
                        .find(|t| match &t.value {
                            TokenFieldType::Token(t) => t.value == input.arg.token_type.value,
                            _ => false,
                        })
                        .is_none()
                    {
                        return Err(CompilationError {
                            stage: crate::compiler::CompilationStage::GenerateRestrictions,
                            exit_code: exitcode::DATAERR,
                            inner: PestError::new_from_span(
                                ErrorVariant::CustomError {
                                    message: format!(
                                        "Invalid comparison between token type {} and property {} on token {}",
                                        input.arg.token_type.value, output.prop, output.arg.token_type.value
                                    ),
                                },
                                span,
                            ),
                        });
                    }

                    let original_key = TokenMetadataKey::try_from(ORIGINAL_ID_KEY.to_vec()).unwrap();
                    let output_metadata_key =
                        TokenMetadataKey::try_from(output.prop.as_bytes().to_vec()).map_err(|_| CompilationError {
                            stage: crate::compiler::CompilationStage::GenerateRestrictions,
                            exit_code: exitcode::DATAERR,
                            inner: PestError::new_from_span(
                                ErrorVariant::CustomError {
                                    message: format!("Property key {} is too long", output.prop),
                                },
                                left.value.prop.span,
                            ),
                        })?;

                    let result = vec![
                        BooleanExpressionSymbol::Restriction(Restriction::MatchArgsMetadataValue {
                            left_arg_type: ArgType::Input,
                            left_index: input.index,
                            left_metadata_key: original_key.clone(),
                            right_arg_type: ArgType::Output,
                            right_index: output.index,
                            right_metadata_key: output_metadata_key.clone(),
                        }),
                        BooleanExpressionSymbol::Restriction(Restriction::ArgHasMetadata {
                            arg_type: ArgType::Input,
                            index: input.index,
                            metadata_key: output_metadata_key.clone(),
                        }),
                        BooleanExpressionSymbol::Restriction(Restriction::MatchArgIdToMetadataValue {
                            left_arg_type: ArgType::Input,
                            left_index: input.index,
                            right_arg_type: ArgType::Output,
                            right_index: output.index,
                            right_metadata_key: output_metadata_key.clone(),
                        }),
                        BooleanExpressionSymbol::Op(BooleanOperator::InhibitionR),
                        BooleanExpressionSymbol::Op(match op {
                            BoolCmp::Eq => BooleanOperator::Xor,
                            BoolCmp::Neq => BooleanOperator::Xnor,
                        }),
                    ];

                    Ok(result)
                }
                Comparison::PropProp { left, op, right } => {
                    let left = find_token_prop(token_decls, fn_decl, &left.value)?;
                    let right = find_token_prop(token_decls, fn_decl, &right.value)?;

                    let (input, output) = match (&left.is_input, &right.is_input) {
                        (true, false) => Ok((left, right)),
                        (false, true) => Ok((right, left)),
                        _ => Err(CompilationError {
                            stage: crate::compiler::CompilationStage::GenerateRestrictions,
                            exit_code: exitcode::DATAERR,
                            inner: PestError::new_from_span(
                                ErrorVariant::CustomError {
                                    message: "Token property comparisons must be between an input and an output".into(),
                                },
                                span,
                            ),
                        }),
                    }?;

                    if input.types.len() != output.types.len()
                        || input
                            .types
                            .iter()
                            .any(|i_t| output.types.iter().find(|o_t| i_t.value == o_t.value).is_none())
                    {
                        return Err(CompilationError {
                            stage: crate::compiler::CompilationStage::GenerateRestrictions,
                            exit_code: exitcode::DATAERR,
                            inner: PestError::new_from_span(
                                ErrorVariant::CustomError {
                                    message: "Cannot compare properties of different types".into(),
                                },
                                span,
                            ),
                        });
                    }

                    let input_key =
                        TokenMetadataKey::try_from(input.prop.as_bytes().to_vec()).map_err(|_| CompilationError {
                            stage: crate::compiler::CompilationStage::GenerateRestrictions,
                            exit_code: exitcode::DATAERR,
                            inner: PestError::new_from_span(
                                ErrorVariant::CustomError {
                                    message: format!("Property key {} is too long", input.prop),
                                },
                                span,
                            ),
                        })?;

                    let output_key =
                        TokenMetadataKey::try_from(output.prop.as_bytes().to_vec()).map_err(|_| CompilationError {
                            stage: crate::compiler::CompilationStage::GenerateRestrictions,
                            exit_code: exitcode::DATAERR,
                            inner: PestError::new_from_span(
                                ErrorVariant::CustomError {
                                    message: format!("Property key {} is too long", output.prop),
                                },
                                span,
                            ),
                        })?;

                    // each property can be a role, metadata or not present and equality must work in each case
                    // first check what is allowed for the input (we know types are same for output)
                    let output_can_be_role = output.types.iter().find(|t| t.value == TokenFieldType::Role).is_some();
                    let output_can_be_none = output.types.iter().find(|t| t.value == TokenFieldType::None).is_some();
                    let output_can_be_metadata = output
                        .types
                        .iter()
                        .find(|t| t.value != TokenFieldType::Role && t.value != TokenFieldType::None)
                        .is_some();

                    let mut result: Vec<RuntimeExpressionSymbol> = Vec::new();
                    let mut check_count = 0;
                    if output_can_be_role {
                        result.push(BooleanExpressionSymbol::Restriction(Restriction::MatchArgsRole {
                            left_arg_type: ArgType::Input,
                            left_index: input.index,
                            left_role_key: input_key.clone(),
                            right_arg_type: ArgType::Output,
                            right_index: output.index,
                            right_role_key: output_key.clone(),
                        }));
                        if output_can_be_none || output_can_be_metadata {
                            result.push(BooleanExpressionSymbol::Restriction(Restriction::ArgHasRole {
                                arg_type: ArgType::Output,
                                index: output.index,
                                role_key: output_key.clone(),
                            }));
                            result.push(BooleanExpressionSymbol::Op(BooleanOperator::ImplicationR));
                        }
                        check_count = check_count + 1;
                    }

                    if output_can_be_none {
                        result.push(BooleanExpressionSymbol::Restriction(Restriction::ArgHasMetadata {
                            arg_type: ArgType::Input,
                            index: input.index,
                            metadata_key: input_key.clone(),
                        }));
                        result.push(BooleanExpressionSymbol::Restriction(Restriction::ArgHasMetadata {
                            arg_type: ArgType::Output,
                            index: output.index,
                            metadata_key: output_key.clone(),
                        }));
                        result.push(BooleanExpressionSymbol::Op(BooleanOperator::Xnor));
                        result.push(BooleanExpressionSymbol::Restriction(Restriction::ArgHasRole {
                            arg_type: ArgType::Input,
                            index: input.index,
                            role_key: input_key.clone(),
                        }));
                        result.push(BooleanExpressionSymbol::Restriction(Restriction::ArgHasRole {
                            arg_type: ArgType::Output,
                            index: output.index,
                            role_key: output_key.clone(),
                        }));
                        result.push(BooleanExpressionSymbol::Op(BooleanOperator::Xnor));
                        result.push(BooleanExpressionSymbol::Op(BooleanOperator::And));
                        check_count = check_count + 1;
                    }

                    if output_can_be_metadata {
                        result.push(BooleanExpressionSymbol::Restriction(
                            Restriction::MatchArgsMetadataValue {
                                left_arg_type: ArgType::Input,
                                left_index: input.index,
                                left_metadata_key: input_key.clone(),
                                right_arg_type: ArgType::Output,
                                right_index: output.index,
                                right_metadata_key: output_key.clone(),
                            },
                        ));
                        if output_can_be_none || output_can_be_role {
                            result.push(BooleanExpressionSymbol::Restriction(Restriction::ArgHasMetadata {
                                arg_type: ArgType::Output,
                                index: output.index,
                                metadata_key: output_key.clone(),
                            }));
                            result.push(BooleanExpressionSymbol::Op(BooleanOperator::ImplicationR));
                        }
                        check_count = check_count + 1;
                    }

                    result.append(&mut vec![
                        BooleanExpressionSymbol::Op(BooleanOperator::And);
                        check_count - 1
                    ]);

                    if op == BoolCmp::Neq {
                        result.append(&mut vec![
                            BooleanExpressionSymbol::Restriction(Restriction::None),
                            BooleanExpressionSymbol::Op(BooleanOperator::NotL),
                        ]);
                    }

                    Ok(result)
                }
                Comparison::PropType { left, op, right } => {
                    let left = find_token_prop(token_decls, fn_decl, &left.value)?;

                    let metadata_key =
                        TokenMetadataKey::try_from(left.prop.as_bytes().to_vec()).map_err(|_| CompilationError {
                            stage: crate::compiler::CompilationStage::GenerateRestrictions,
                            exit_code: exitcode::DATAERR,
                            inner: PestError::new_from_span(
                                ErrorVariant::CustomError {
                                    message: format!("Property key {} is too long", left.prop),
                                },
                                span,
                            ),
                        })?;

                    let mut result = match right.value {
                        TypeCmpType::None => {
                            vec![
                                BooleanExpressionSymbol::Restriction(match left.is_input {
                                    true => Restriction::ArgHasMetadata {
                                        arg_type: ArgType::Input,
                                        index: left.index,
                                        metadata_key: metadata_key.clone(),
                                    },
                                    false => Restriction::ArgHasMetadata {
                                        arg_type: ArgType::Output,
                                        index: left.index,
                                        metadata_key: metadata_key.clone(),
                                    },
                                }),
                                BooleanExpressionSymbol::Restriction(Restriction::None),
                                BooleanExpressionSymbol::Op(BooleanOperator::NotL),
                            ]
                        }
                        TypeCmpType::File => vec![BooleanExpressionSymbol::Restriction(match left.is_input {
                            true => Restriction::FixedArgMetadataValueType {
                                arg_type: ArgType::Input,
                                index: left.index,
                                metadata_key: metadata_key.clone(),
                                metadata_value_type: MetadataValueType::File,
                            },
                            false => Restriction::FixedArgMetadataValueType {
                                arg_type: ArgType::Output,
                                index: left.index,
                                metadata_key: metadata_key.clone(),
                                metadata_value_type: MetadataValueType::File,
                            },
                        })],
                        TypeCmpType::Role => vec![BooleanExpressionSymbol::Restriction(match left.is_input {
                            true => Restriction::ArgHasRole {
                                arg_type: ArgType::Input,
                                index: left.index,
                                role_key: metadata_key,
                            },
                            false => Restriction::ArgHasRole {
                                arg_type: ArgType::Output,
                                index: left.index,
                                role_key: metadata_key,
                            },
                        })],
                        TypeCmpType::Literal => vec![BooleanExpressionSymbol::Restriction(match left.is_input {
                            true => Restriction::FixedArgMetadataValueType {
                                arg_type: ArgType::Input,
                                index: left.index,
                                metadata_key: metadata_key.clone(),
                                metadata_value_type: MetadataValueType::Literal,
                            },
                            false => Restriction::FixedArgMetadataValueType {
                                arg_type: ArgType::Output,
                                index: left.index,
                                metadata_key: metadata_key.clone(),
                                metadata_value_type: MetadataValueType::Literal,
                            },
                        })],
                        TypeCmpType::Integer => vec![BooleanExpressionSymbol::Restriction(match left.is_input {
                            true => Restriction::FixedArgMetadataValueType {
                                arg_type: ArgType::Input,
                                index: left.index,
                                metadata_key: metadata_key.clone(),
                                metadata_value_type: MetadataValueType::Integer,
                            },
                            false => Restriction::FixedArgMetadataValueType {
                                arg_type: ArgType::Output,
                                index: left.index,
                                metadata_key: metadata_key.clone(),
                                metadata_value_type: MetadataValueType::Integer,
                            },
                        })],
                        TypeCmpType::Token => vec![BooleanExpressionSymbol::Restriction(match left.is_input {
                            true => Restriction::FixedArgMetadataValueType {
                                arg_type: ArgType::Input,
                                index: left.index,
                                metadata_key: metadata_key.clone(),
                                metadata_value_type: MetadataValueType::TokenId,
                            },
                            false => Restriction::FixedArgMetadataValueType {
                                arg_type: ArgType::Output,
                                index: left.index,
                                metadata_key: metadata_key.clone(),
                                metadata_value_type: MetadataValueType::TokenId,
                            },
                        })],
                    };

                    if op == TypeCmp::Isnt {
                        result.append(&mut vec![
                            BooleanExpressionSymbol::Restriction(Restriction::None),
                            BooleanExpressionSymbol::Op(BooleanOperator::NotL),
                        ]);
                    }

                    Ok(result)
                }
            }
        }
        ExpressionTree::Not(exp) => {
            let mut program = transform_condition_to_program(fn_decl, token_decls, *exp)?;
            program.append(&mut vec![
                BooleanExpressionSymbol::Restriction(Restriction::None),
                BooleanExpressionSymbol::Op(BooleanOperator::NotL),
            ]);
            Ok(program)
        }
        ExpressionTree::Node { left, op, right } => {
            let mut program = transform_condition_to_program(fn_decl, token_decls, *left)?;
            program.append(&mut transform_condition_to_program(fn_decl, token_decls, *right)?);
            program.push(BooleanExpressionSymbol::Op(match op {
                BoolOp::And => BooleanOperator::And,
                BoolOp::Or => BooleanOperator::Or,
                BoolOp::Xor => BooleanOperator::Xor,
            }));
            Ok(program)
        }
    }
}
