use std::sync::Arc;

use super::types::*;

use crate::{
    errors::{produce_unexpected_pair_error, CompilationError, CompilationStage, ErrorVariant, PestError},
    parser::Rule,
};

fn parse_ident<'a>(pair: pest::iterators::Pair<'a, Rule>) -> Result<AstNode<'a, &'a str>, CompilationError> {
    Ok(AstNode {
        value: pair.as_str(),
        span: pair.as_span(),
    })
}

fn parse_literal<'a>(pair: pest::iterators::Pair<'a, Rule>) -> Result<AstNode<'a, &'a str>, CompilationError> {
    let string = pair.into_inner().next().unwrap(); // string
    let inner = string.into_inner().next().unwrap(); // inner
    Ok(AstNode {
        value: inner.as_str(),
        span: inner.as_span(),
    })
}

fn parse_integer<'a>(pair: pest::iterators::Pair<'a, Rule>) -> Result<AstNode<'a, i128>, CompilationError> {
    let integer_value_string = pair.into_inner().next().unwrap(); // integer_value
    let parsed_integer = integer_value_string.as_str().parse::<i128>();

    match parsed_integer {
        Ok(val) => Ok(AstNode {
            value: val,
            span: integer_value_string.as_span(),
        }),
        Err(_) => Err(CompilationError {
            stage: CompilationStage::BuildAst,
            exit_code: exitcode::DATAERR,
            inner: PestError::new_from_span(
                ErrorVariant::CustomError {
                    message: "Error parsing integer".into(),
                },
                integer_value_string.as_span(),
            ),
        }),
    }
}

fn parse_role_literal<'a>(pair: pest::iterators::Pair<'a, Rule>) -> Result<AstNode<'a, RoleLit>, CompilationError> {
    let role_value_string = pair.into_inner().next().unwrap(); // integer_value

    match role_value_string.as_rule() {
        Rule::root => Ok(AstNode {
            value: RoleLit::Root,
            span: role_value_string.as_span(),
        }),
        _ => produce_unexpected_pair_error(role_value_string),
    }
}

fn parse_token_prop_type(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<TokenFieldType>, CompilationError> {
    let span = pair.as_span();
    let field_type = match pair.as_rule() {
        Rule::file => Ok(TokenFieldType::File),
        Rule::literal => Ok(TokenFieldType::Literal),
        Rule::integer => Ok(TokenFieldType::Integer),
        Rule::role => Ok(TokenFieldType::Role),
        Rule::none => Ok(TokenFieldType::None),
        Rule::literal_value => Ok(TokenFieldType::LiteralValue(parse_literal(pair)?)),
        Rule::integer_value => Ok(TokenFieldType::IntegerValue(parse_integer(pair)?)),
        Rule::ident => Ok(TokenFieldType::Token(AstNode {
            span: pair.as_span(),
            value: pair.as_str(),
        })),
        _ => produce_unexpected_pair_error(pair),
    }?;

    Ok(AstNode {
        span,
        value: field_type,
    })
}

fn parse_token_prop_field(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<TokenPropDecl>, CompilationError> {
    match pair.as_rule() {
        Rule::field => {
            let span = pair.as_span();
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                span,
                value: TokenPropDecl {
                    name: parse_ident(pairs.next().unwrap())?,
                    types: pairs
                        .next()
                        .unwrap()
                        .into_inner()
                        .into_iter()
                        .map(parse_token_prop_type)
                        .collect::<Result<Arc<[_]>, _>>()?,
                },
            })
        }
        _ => produce_unexpected_pair_error(pair),
    }
}

fn parse_token_props(
    pair: pest::iterators::Pair<Rule>,
) -> Result<AstNode<Arc<[AstNode<TokenPropDecl>]>>, CompilationError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::properties => Ok(AstNode {
            span,
            value: pair
                .into_inner()
                .into_iter()
                .map(parse_token_prop_field)
                .collect::<Result<Arc<[_]>, _>>()?,
        }),
        _ => produce_unexpected_pair_error(pair),
    }
}

fn parse_token_decl(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<TokenDecl>, CompilationError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::token_decl => {
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                span,
                value: TokenDecl {
                    name: parse_ident(pairs.next().unwrap())?,
                    props: parse_token_props(pairs.next().unwrap())?,
                },
            })
        }
        _ => produce_unexpected_pair_error(pair),
    }
}

fn parse_fn_input(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<FnArg>, CompilationError> {
    if pair.as_rule() != Rule::fn_decl_input {
        return produce_unexpected_pair_error(pair);
    }

    let span = pair.as_span();
    let mut pairs = pair.into_inner();
    let first = pairs.next().unwrap();
    let second = pairs.next().unwrap();

    match second.as_rule() {
        Rule::reference_amp => Ok(AstNode {
            value: FnArg {
                is_reference: true,
                name: parse_ident(first)?,
                token_type: parse_ident(pairs.next().unwrap())?,
            },
            span,
        }),
        Rule::ident => Ok(AstNode {
            value: FnArg {
                is_reference: false,
                name: parse_ident(first)?,
                token_type: parse_ident(second)?,
            },
            span,
        }),
        _ => produce_unexpected_pair_error(second),
    }
}

fn parse_fn_output(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<FnArg>, CompilationError> {
    match pair.as_rule() {
        Rule::fn_decl_output => {
            let span = pair.as_span();
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                value: FnArg {
                    is_reference: false,
                    name: parse_ident(pairs.next().unwrap())?,
                    token_type: parse_ident(pairs.next().unwrap())?,
                },
                span,
            })
        }
        _ => produce_unexpected_pair_error(pair),
    }
}

fn parse_fn_decl_args(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<Arc<[AstNode<FnArg>]>>, CompilationError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::fn_decl_input_list => {
            let args = pair.into_inner();
            Ok(AstNode {
                value: args.into_iter().map(parse_fn_input).collect::<Result<Arc<[_]>, _>>()?,
                span,
            })
        }
        Rule::fn_decl_output_list => {
            let args = pair.into_inner();
            Ok(AstNode {
                value: args.into_iter().map(parse_fn_output).collect::<Result<Arc<[_]>, _>>()?,
                span,
            })
        }
        _ => produce_unexpected_pair_error(pair),
    }
}

fn parse_fn_vis(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<FnVis>, CompilationError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::vis => Ok(AstNode {
            value: match pair.as_str() {
                "" => FnVis::Private,
                "priv" => FnVis::Private,
                "pub" => FnVis::Public,
                _ => {
                    return Err(CompilationError {
                        stage: CompilationStage::BuildAst,
                        exit_code: exitcode::DATAERR,
                        inner: PestError::new_from_span(
                            ErrorVariant::CustomError {
                                message: "visibility if specified must be pub/priv".into(),
                            },
                            span,
                        ),
                    })
                }
            },
            span,
        }),
        _ => produce_unexpected_pair_error(pair),
    }
}

fn parse_bool_cmp_op(pair: pest::iterators::Pair<Rule>) -> Result<BoolCmp, CompilationError> {
    match pair.as_rule() {
        Rule::eq => Ok(BoolCmp::Eq),
        Rule::neq => Ok(BoolCmp::Neq),
        _ => produce_unexpected_pair_error(pair),
    }
}

fn parse_type_cmp_op(pair: pest::iterators::Pair<Rule>) -> Result<TypeCmp, CompilationError> {
    match pair.as_rule() {
        Rule::is => Ok(TypeCmp::Is),
        Rule::isnt => Ok(TypeCmp::Isnt),
        _ => produce_unexpected_pair_error(pair),
    }
}

fn parse_type_cmp_type(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<TypeCmpType>, CompilationError> {
    let span = pair.as_span();
    let value = match pair.as_rule() {
        Rule::none => Ok(TypeCmpType::None),
        Rule::role => Ok(TypeCmpType::Role),
        Rule::literal => Ok(TypeCmpType::Literal),
        Rule::file => Ok(TypeCmpType::File),
        _ => produce_unexpected_pair_error(pair),
    }?;
    Ok(AstNode { value, span })
}

fn parse_ident_prop<'a>(pair: pest::iterators::Pair<'a, Rule>) -> Result<AstNode<'a, TokenProp<'a>>, CompilationError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::ident_prop => {
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                value: TokenProp {
                    token: parse_ident(pairs.next().unwrap())?,
                    prop: parse_ident(pairs.next().unwrap())?,
                },
                span,
            })
        }
        _ => produce_unexpected_pair_error(pair),
    }
}

fn parse_fn_inv_args<'a>(
    pair: pest::iterators::Pair<Rule>,
) -> Result<AstNode<'a, Arc<[AstNode<&'a str>]>>, CompilationError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::fn_args => {
            let args = pair.into_inner();
            Ok(AstNode {
                value: args.into_iter().map(parse_ident).collect::<Result<Arc<[_]>, _>>()?,
                span,
            })
        }
        _ => produce_unexpected_pair_error(pair),
    }
}

fn parse_bool_cmp(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<Comparison>, CompilationError> {
    let pair_cmp_type = pair.as_rule();
    let span = pair.as_span();
    match pair_cmp_type {
        Rule::fn_cmp => {
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                value: Comparison::Fn {
                    name: parse_ident(pairs.next().unwrap())?,
                    inputs: parse_fn_inv_args(pairs.next().unwrap())?,
                    outputs: parse_fn_inv_args(pairs.next().unwrap())?,
                },
                span,
            })
        }
        Rule::prop_prop_cmp => {
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                value: Comparison::PropProp {
                    left: parse_ident_prop(pairs.next().unwrap())?,
                    op: parse_bool_cmp_op(pairs.next().unwrap())?,
                    right: parse_ident_prop(pairs.next().unwrap())?,
                },
                span,
            })
        }
        Rule::prop_lit_cmp => {
            let mut pairs = pair.into_inner();

            let left = parse_ident_prop(pairs.next().unwrap())?;
            let op = parse_bool_cmp_op(pairs.next().unwrap())?;
            let right = parse_literal(pairs.next().unwrap())?; // literal_value

            Ok(AstNode {
                value: Comparison::PropLit { left, op, right },
                span,
            })
        }
        Rule::prop_int_cmp => {
            let mut pairs = pair.into_inner();

            let left = parse_ident_prop(pairs.next().unwrap())?;
            let op = parse_bool_cmp_op(pairs.next().unwrap())?;
            let right = parse_integer(pairs.next().unwrap())?; // literal_value

            Ok(AstNode {
                value: Comparison::PropInt { left, op, right },
                span,
            })
        }
        Rule::prop_sender_cmp => {
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                value: Comparison::PropSender {
                    left: parse_ident_prop(pairs.next().unwrap())?,
                    op: parse_bool_cmp_op(pairs.next().unwrap())?,
                },
                span,
            })
        }
        Rule::sender_role_cmp => {
            let mut pairs = pair.into_inner();
            let _sender = pairs.next(); // lhs is sender
            Ok(AstNode {
                value: Comparison::SenderRole {
                    op: parse_bool_cmp_op(pairs.next().unwrap())?,
                    right: parse_role_literal(pairs.next().unwrap())?,
                },
                span,
            })
        }
        Rule::prop_ident_cmp => {
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                value: Comparison::PropToken {
                    left: parse_ident_prop(pairs.next().unwrap())?,
                    op: parse_bool_cmp_op(pairs.next().unwrap())?,
                    right: parse_ident(pairs.next().unwrap())?,
                },
                span,
            })
        }
        Rule::ident_ident_cmp => {
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                value: Comparison::TokenToken {
                    left: parse_ident(pairs.next().unwrap())?,
                    op: parse_bool_cmp_op(pairs.next().unwrap())?,
                    right: parse_ident(pairs.next().unwrap())?,
                },
                span,
            })
        }
        Rule::prop_type_cmp => {
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                value: Comparison::PropType {
                    left: parse_ident_prop(pairs.next().unwrap())?,
                    op: parse_type_cmp_op(pairs.next().unwrap())?,
                    right: parse_type_cmp_type(pairs.next().unwrap())?,
                },
                span,
            })
        }
        _ => produce_unexpected_pair_error(pair),
    }
}

fn partition_pairs_by_op<'a, I>(
    pairs: I,
    rule: Rule,
    op: BoolOp,
) -> Result<Option<ExpressionTree<'a>>, CompilationError>
where
    I: IntoIterator<Item = pest::iterators::Pair<'a, Rule>>,
    <I as IntoIterator>::Item: Clone,
    <I as IntoIterator>::IntoIter: Clone,
{
    let pairs = pairs.into_iter();
    let mut iter = pairs.clone();
    let left: Vec<_> = iter.by_ref().take_while(|p| p.as_rule() != rule.clone()).collect();
    let right: Vec<_> = iter.collect();

    match right.len() != 0 {
        true => Ok(Some(ExpressionTree::Node {
            left: Box::new(parse_exp_tree(left)?),
            op,
            right: Box::new(parse_exp_tree(right)?),
        })),
        false => Ok(None),
    }
}

fn parse_exp_tree<'a, I>(pairs: I) -> Result<ExpressionTree<'a>, CompilationError>
where
    I: IntoIterator<Item = pest::iterators::Pair<'a, Rule>>,
    <I as IntoIterator>::Item: Clone,
    <I as IntoIterator>::IntoIter: Clone,
{
    let mut pairs = pairs.into_iter();
    let split = partition_pairs_by_op(pairs.clone(), Rule::bool_op_and, BoolOp::And)?;
    if split.is_some() {
        return Ok(split.unwrap());
    }
    let split = partition_pairs_by_op(pairs.clone(), Rule::bool_op_xor, BoolOp::Xor)?;
    if split.is_some() {
        return Ok(split.unwrap());
    }
    let split = partition_pairs_by_op(pairs.clone(), Rule::bool_op_or, BoolOp::Or)?;
    if split.is_some() {
        return Ok(split.unwrap());
    }

    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::unary_not => Ok(ExpressionTree::Not(Box::new(parse_exp_tree(
            pairs.next().unwrap().into_inner(),
        )?))),
        Rule::expr => parse_exp_tree(pair.into_inner()),
        _ => Ok(ExpressionTree::Leaf(parse_bool_cmp(pair)?)),
    }
}

fn parse_fn_conditions(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<Vec<ExpressionTree>>, CompilationError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::expr_list => {
            let inner = pair.into_inner();
            Ok(AstNode {
                value: inner
                    .into_iter()
                    .map(|pair| parse_exp_tree(Box::new(pair.into_inner())))
                    .collect::<Result<Vec<_>, _>>()?,
                span,
            })
        }
        _ => produce_unexpected_pair_error(pair),
    }
}

fn parse_fn_decl(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<FnDecl>, CompilationError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::fn_decl => {
            let mut pair = pair.into_inner();
            let vis = pair.next().unwrap();
            let name = pair.next().unwrap();
            let inputs = pair.next().unwrap();
            let outputs = pair.next().unwrap();
            let conditions = pair.next().unwrap();
            Ok(AstNode {
                value: FnDecl {
                    visibility: parse_fn_vis(vis)?,
                    name: AstNode {
                        value: name.as_str(),
                        span: name.as_span(),
                    },
                    inputs: parse_fn_decl_args(inputs)?,
                    outputs: parse_fn_decl_args(outputs)?,
                    conditions: parse_fn_conditions(conditions)?,
                },
                span,
            })
        }
        _ => produce_unexpected_pair_error(pair),
    }
}

pub fn parse_ast(pairs: pest::iterators::Pairs<Rule>) -> Result<Vec<AstNode<AstRoot>>, CompilationError> {
    pairs
        .into_iter()
        .filter_map(|pair| match pair.as_rule() {
            Rule::token_decl => {
                let span = pair.as_span();
                let node = parse_token_decl(pair).map(|token_decl| AstNode {
                    span,
                    value: AstRoot::TokenDecl(token_decl),
                });
                Some(node)
            }
            Rule::fn_decl => {
                let fn_span = pair.as_span();
                let node = parse_fn_decl(pair).map(|fn_decl| AstNode {
                    span: fn_span,
                    value: AstRoot::FnDecl(fn_decl),
                });
                Some(node)
            }
            Rule::EOI => None,
            _ => Some(produce_unexpected_pair_error(pair)),
        })
        .collect()
}
