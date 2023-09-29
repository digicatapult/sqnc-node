use crate::*;

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
            value: pair.into_inner().as_str()
        })),
        _ => produce_unexpected_pair_error(pair)
    }?;

    Ok(AstNode {
        span,
        value: field_type
    })
}

fn parse_token_prop_field(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<TokenProp>, AstBuildError> {
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
                value: TokenProp {
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

fn parse_token_props(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<Vec<AstNode<TokenProp>>>, AstBuildError> {
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

fn parse_fn_decl(pair: pest::iterators::Pair<Rule>) -> Result<AstNode<FnDecl>, AstBuildError> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::fn_decl => {
            let mut pair = pair.into_inner();
            let vis = pair.next().unwrap();
            let name = pair.next().unwrap();
            Ok(AstNode {
                value: FnDecl {
                    visibility: AstNode {
                        value: match vis.as_str() {
                            "" => FnVis::Private,
                            "priv" => FnVis::Private,
                            "pub" => FnVis::Public,
                            _ => {
                                return Err(AstBuildError {
                                    message: "visibility if specified must be pub/priv".into(),
                                    span: vis.as_span()
                                })
                            }
                        },
                        span: vis.as_span()
                    },
                    name: AstNode {
                        value: name.as_str(),
                        span: name.as_span()
                    }
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
