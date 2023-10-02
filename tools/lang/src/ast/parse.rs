use super::*;

use crate::parser::Rule;

fn parse_token_prop_type(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<TokenFieldType>, AstBuildError> {
    let span = pair.as_span();
    let field_type = match pair.as_rule() {
        Rule::file => Ok(TokenFieldType::File),
        Rule::literal => Ok(TokenFieldType::Literal),
        Rule::role => Ok(TokenFieldType::Role),
        Rule::none => Ok(TokenFieldType::None),
        Rule::literal_value => {
            let literal_string = pair.into_inner().next().unwrap();
            let span = literal_string.as_span();
            let literal_val = literal_string.into_inner();
            Ok(TokenFieldType::LiteralValue(AstNode {
                value: literal_val.as_str(),
                span
            }))
        }
        Rule::ident => Ok(TokenFieldType::Token(AstNode {
            span: pair.as_span(),
            value: pair.as_str()
        })),
        _ => produce_unexpected_pair_error(pair)
    }?;

    Ok(AstNode {
        span,
        value: field_type
    })
}

fn parse_token_prop_field(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<TokenPropDecl>, AstBuildError> {
    match pair.as_rule() {
        Rule::field => {
            let span = pair.as_span();
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let valid_types = pair
                .next()
                .unwrap()
                .into_inner()
                .into_iter()
                .map(parse_token_prop_type)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(AstNode {
                span,
                value: TokenPropDecl {
                    name: AstNode {
                        value: name.as_str(),
                        span: name.as_span()
                    },
                    types: valid_types
                }
            })
        }
        _ => produce_unexpected_pair_error(pair)
    }
}

fn parse_token_props(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<Vec<AstNode<TokenPropDecl>>>, AstBuildError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::properties => Ok(AstNode {
            span,
            value: pair
                .into_inner()
                .into_iter()
                .map(parse_token_prop_field)
                .collect::<Result<Vec<_>, _>>()?
        }),
        _ => produce_unexpected_pair_error(pair)
    }
}

fn parse_token_decl(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<TokenDecl>, AstBuildError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::struct_decl => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let props = parse_token_props(pair.next().unwrap())?;

            Ok(AstNode {
                span,
                value: TokenDecl {
                    name: AstNode {
                        value: name.as_str(),
                        span: name.as_span()
                    },
                    props
                }
            })
        }
        _ => produce_unexpected_pair_error(pair)
    }
}

fn parse_fn_arg(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<FnArg>, AstBuildError> {
    match pair.as_rule() {
        Rule::fn_decl_arg => {
            let span = pair.as_span();
            let mut pairs = pair.into_inner();
            let name = pairs.next().unwrap();
            let token_type = pairs.next().unwrap();
            Ok(AstNode {
                value: FnArg {
                    name: AstNode {
                        value: name.as_str(),
                        span: name.as_span()
                    },
                    token_type: AstNode {
                        value: token_type.as_str(),
                        span: token_type.as_span()
                    }
                },
                span
            })
        }
        _ => produce_unexpected_pair_error(pair)
    }
}

fn parse_fn_decl_args(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<Vec<AstNode<FnArg>>>, AstBuildError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::fn_decl_arg_list => {
            let args = pair.into_inner();
            Ok(AstNode {
                value: args.into_iter().map(parse_fn_arg).collect::<Result<Vec<_>, _>>()?,
                span
            })
        }
        _ => produce_unexpected_pair_error(pair)
    }
}

fn parse_fn_vis(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<FnVis>, AstBuildError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::vis => Ok(AstNode {
            value: match pair.as_str() {
                "" => FnVis::Private,
                "priv" => FnVis::Private,
                "pub" => FnVis::Public,
                _ => {
                    return Err(AstBuildError {
                        stage: crate::CompilationStage::BuildAst,
                        inner: PestError::new_from_span(
                            ErrorVariant::CustomError {
                                message: "visibility if specified must be pub/priv".into()
                            },
                            span
                        )
                    })
                }
            },
            span
        }),
        _ => produce_unexpected_pair_error(pair)
    }
}

fn parse_bool_cmp_op(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<BoolCmp>, AstBuildError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::eq => Ok(AstNode {
            value: BoolCmp::Eq,
            span
        }),
        Rule::neq => Ok(AstNode {
            value: BoolCmp::Neq,
            span
        }),
        _ => produce_unexpected_pair_error(pair)
    }
}

fn parse_type_cmp_op(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<TypeCmp>, AstBuildError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::is => Ok(AstNode {
            value: TypeCmp::Is,
            span
        }),
        Rule::isnt => Ok(AstNode {
            value: TypeCmp::Isnt,
            span
        }),
        _ => produce_unexpected_pair_error(pair)
    }
}

fn parse_ident_prop<'a>(pair: pest::iterators::Pair<'a, Rule>) -> Result<AstNode<TokenProp<'a>>, AstBuildError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::ident_prop => {
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                value: TokenProp {
                    token: pairs.next().unwrap().as_str(),
                    prop: pairs.next().unwrap().as_str()
                },
                span
            })
        }
        _ => produce_unexpected_pair_error(pair)
    }
}

fn parse_fn_inv_args<'a>(
    pair: pest::iterators::Pair<Rule>
) -> Result<AstNode<'a, Vec<AstNode<&'a str>>>, AstBuildError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::fn_args => {
            let args = pair.into_inner();
            Ok(AstNode {
                value: args
                    .into_iter()
                    .map(|p| AstNode {
                        value: p.as_str(),
                        span: p.as_span()
                    })
                    .collect::<Vec<_>>(),
                span
            })
        }
        _ => produce_unexpected_pair_error(pair)
    }
}

fn parse_bool_cmp(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<Comparison>, AstBuildError> {
    let cmp_type = pair.as_rule();
    let span = pair.as_span();
    match cmp_type {
        Rule::fn_cmp => {
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                value: Comparison::Fn {
                    name: pairs.next().unwrap().as_str(),
                    inputs: parse_fn_inv_args(pairs.next().unwrap())?,
                    outputs: parse_fn_inv_args(pairs.next().unwrap())?
                },
                span
            })
        }
        Rule::prop_prop_cmp => {
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                value: Comparison::PropProp {
                    left: parse_ident_prop(pairs.next().unwrap())?,
                    op: parse_bool_cmp_op(pairs.next().unwrap())?,
                    right: parse_ident_prop(pairs.next().unwrap())?
                },
                span
            })
        }
        Rule::prop_lit_cmp => {
            let mut pairs = pair.into_inner();

            let left = parse_ident_prop(pairs.next().unwrap())?;
            let op = parse_bool_cmp_op(pairs.next().unwrap())?;
            let right = pairs.next().unwrap(); // literal_value
            let right = right.into_inner().next().unwrap(); // string
            let right = right.into_inner().next().unwrap().as_str(); // inner

            Ok(AstNode {
                value: Comparison::PropLit { left, op, right },
                span
            })
        }
        Rule::prop_sender_cmp => {
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                value: Comparison::PropSender {
                    left: parse_ident_prop(pairs.next().unwrap())?,
                    op: parse_bool_cmp_op(pairs.next().unwrap())?
                },
                span
            })
        }
        Rule::prop_ident_cmp => {
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                value: Comparison::PropToken {
                    left: parse_ident_prop(pairs.next().unwrap())?,
                    op: parse_bool_cmp_op(pairs.next().unwrap())?,
                    right: pairs.next().unwrap().as_str()
                },
                span
            })
        }
        Rule::ident_ident_cmp => {
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                value: Comparison::TokenToken {
                    left: pairs.next().unwrap().as_str(),
                    op: parse_bool_cmp_op(pairs.next().unwrap())?,
                    right: pairs.next().unwrap().as_str()
                },
                span
            })
        }
        Rule::prop_type_cmp => {
            let mut pairs = pair.into_inner();
            Ok(AstNode {
                value: Comparison::PropType {
                    left: parse_ident_prop(pairs.next().unwrap())?,
                    op: parse_type_cmp_op(pairs.next().unwrap())?,
                    right: pairs.next().unwrap().as_str()
                },
                span
            })
        }
        _ => produce_unexpected_pair_error(pair)
    }
}

fn partition_pairs_by_op<'a, I>(pairs: I, rule: Rule, op: BoolOp) -> Result<Option<ExpressionTree<'a>>, AstBuildError>
where
    I: IntoIterator<Item = pest::iterators::Pair<'a, Rule>>,
    <I as IntoIterator>::Item: Clone,
    <I as IntoIterator>::IntoIter: Clone
{
    let mut pairs = pairs.into_iter();
    let left: Vec<_> = pairs.by_ref().take_while(|p| p.as_rule() != rule.clone()).collect();
    let right: Vec<_> = pairs.collect();

    match right.len() != 0 {
        true => Ok(Some(ExpressionTree::Node {
            left: Box::new(parse_exp_tree(left)?),
            op,
            right: Box::new(parse_exp_tree(right)?)
        })),
        false => Ok(None)
    }
}

fn parse_exp_tree<'a, I>(pairs: I) -> Result<ExpressionTree<'a>, AstBuildError>
where
    I: IntoIterator<Item = pest::iterators::Pair<'a, Rule>>,
    <I as IntoIterator>::Item: Clone,
    <I as IntoIterator>::IntoIter: Clone
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
            pairs.next().unwrap().into_inner()
        )?))),
        Rule::expr => parse_exp_tree(pair.into_inner()),
        _ => Ok(ExpressionTree::Leaf(parse_bool_cmp(pair)?))
    }
}

fn parse_fn_conditions(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<Vec<ExpressionTree>>, AstBuildError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::expr_list => {
            let inner = pair.into_inner();
            Ok(AstNode {
                value: inner
                    .into_iter()
                    .map(|pair| parse_exp_tree(Box::new(pair.into_inner())))
                    .collect::<Result<Vec<_>, _>>()?,
                span
            })
        }
        _ => produce_unexpected_pair_error(pair)
    }
}

fn parse_fn_decl(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<FnDecl>, AstBuildError> {
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
                        span: name.as_span()
                    },
                    inputs: parse_fn_decl_args(inputs)?,
                    outputs: parse_fn_decl_args(outputs)?,
                    conditions: parse_fn_conditions(conditions)?
                },
                span
            })
        }
        _ => produce_unexpected_pair_error(pair)
    }
}

pub fn parse_ast(pairs: pest::iterators::Pairs<Rule>) -> Result<Vec<AstNode<AstRoot>>, AstBuildError> {
    pairs
        .into_iter()
        .filter_map(|pair| match pair.as_rule() {
            Rule::struct_decl => {
                let span = pair.as_span();
                let node = parse_token_decl(pair).map(|token_decl| AstNode {
                    span,
                    value: AstRoot::TokenDecl(token_decl)
                });
                Some(node)
            }
            Rule::fn_decl => {
                let fn_span = pair.as_span();
                let node = parse_fn_decl(pair).map(|fn_decl| AstNode {
                    span: fn_span,
                    value: AstRoot::FnDecl(fn_decl)
                });
                Some(node)
            }
            Rule::EOI => None,
            _ => panic!()
        })
        .collect()
}
