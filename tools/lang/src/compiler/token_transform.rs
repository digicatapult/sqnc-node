use crate::{
    ast::types::{
        AstNode, BoolCmp, BoolOp, Comparison, ExpressionTree, TokenDecl, TokenFieldType, TokenProp, TokenPropDecl,
        TypeCmp, TypeCmpType,
    },
    errors::CompilationError,
};

pub fn token_decl_to_conditions<'a>(
    token_name: AstNode<'a, &'a str>,
    token_decl: &TokenDecl<'a>,
) -> Result<Vec<ExpressionTree<'a>>, CompilationError> {
    Ok(token_decl
        .props
        .value
        .iter()
        .filter_map(|node| {
            let TokenPropDecl {
                name: field_name,
                types,
            } = &node.value;
            types.into_iter().fold(None, |acc, field_type| {
                let token_prop_node = AstNode {
                    value: TokenProp {
                        token: token_name.clone(),
                        prop: field_name.clone(),
                    },
                    span: node.span,
                };
                let leaf = match &field_type.value {
                    TokenFieldType::None => Comparison::PropType {
                        left: token_prop_node,
                        op: TypeCmp::Is,
                        right: AstNode {
                            value: TypeCmpType::None,
                            span: field_type.span,
                        },
                    },
                    TokenFieldType::File => Comparison::PropType {
                        left: token_prop_node,
                        op: TypeCmp::Is,
                        right: AstNode {
                            value: TypeCmpType::File,
                            span: field_type.span,
                        },
                    },
                    TokenFieldType::Role => Comparison::PropType {
                        left: token_prop_node,
                        op: TypeCmp::Is,
                        right: AstNode {
                            value: TypeCmpType::Role,
                            span: field_type.span,
                        },
                    },
                    TokenFieldType::Literal => Comparison::PropType {
                        left: token_prop_node,
                        op: TypeCmp::Is,
                        right: AstNode {
                            value: TypeCmpType::Literal,
                            span: field_type.span,
                        },
                    },
                    TokenFieldType::LiteralValue(v) => Comparison::PropLit {
                        left: token_prop_node,
                        op: BoolCmp::Eq,
                        right: v.clone(),
                    },
                    TokenFieldType::Token(_) => Comparison::PropType {
                        left: token_prop_node,
                        op: TypeCmp::Is,
                        right: AstNode {
                            value: TypeCmpType::Token,
                            span: field_type.span,
                        },
                    },
                };

                match acc {
                    None => Some(ExpressionTree::Leaf(AstNode {
                        value: leaf,
                        span: node.span,
                    })),
                    Some(acc) => Some(ExpressionTree::Node {
                        left: Box::new(ExpressionTree::Leaf(AstNode {
                            value: leaf,
                            span: node.span,
                        })),
                        op: BoolOp::Or,
                        right: Box::new(acc),
                    }),
                }
            })
        })
        .collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::token_decl_to_conditions;
    use crate::ast::types::{
        AstNode, BoolCmp, BoolOp, Comparison, ExpressionTree, TokenDecl, TokenFieldType, TokenProp, TokenPropDecl,
        TypeCmp, TypeCmpType,
    };

    fn to_ast_node<'a, V>(value: V) -> AstNode<'a, V> {
        let span = pest::Span::new("", 0, 0).unwrap();
        AstNode { value, span }
    }

    #[test]
    fn single_prop_none() {
        let token_name = to_ast_node("test");
        let prop_name = to_ast_node("prop");
        let token_decl = TokenDecl {
            name: to_ast_node("token"),
            props: to_ast_node(Arc::new([to_ast_node(TokenPropDecl {
                name: prop_name.clone(),
                types: Arc::new([to_ast_node(TokenFieldType::None)]),
            })])),
        };
        let result = token_decl_to_conditions(token_name.clone(), &token_decl);

        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result.unwrap(),
            vec![ExpressionTree::Leaf(to_ast_node(Comparison::PropType {
                left: to_ast_node(TokenProp {
                    token: token_name,
                    prop: prop_name
                }),
                op: TypeCmp::Is,
                right: to_ast_node(TypeCmpType::None)
            }))]
        );
    }

    #[test]
    fn single_prop_file() {
        let token_name = to_ast_node("test");
        let prop_name = to_ast_node("prop");
        let token_decl = TokenDecl {
            name: to_ast_node("token"),
            props: to_ast_node(Arc::new([to_ast_node(TokenPropDecl {
                name: prop_name.clone(),
                types: Arc::new([to_ast_node(TokenFieldType::File)]),
            })])),
        };
        let result = token_decl_to_conditions(token_name.clone(), &token_decl);

        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result.unwrap(),
            vec![ExpressionTree::Leaf(to_ast_node(Comparison::PropType {
                left: to_ast_node(TokenProp {
                    token: token_name,
                    prop: prop_name
                }),
                op: TypeCmp::Is,
                right: to_ast_node(TypeCmpType::File)
            }))]
        );
    }

    #[test]
    fn single_prop_role() {
        let token_name = to_ast_node("test");
        let prop_name = to_ast_node("prop");
        let token_decl = TokenDecl {
            name: to_ast_node("token"),
            props: to_ast_node(Arc::new([to_ast_node(TokenPropDecl {
                name: prop_name.clone(),
                types: Arc::new([to_ast_node(TokenFieldType::Role)]),
            })])),
        };
        let result = token_decl_to_conditions(token_name.clone(), &token_decl);

        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result.unwrap(),
            vec![ExpressionTree::Leaf(to_ast_node(Comparison::PropType {
                left: to_ast_node(TokenProp {
                    token: token_name,
                    prop: prop_name
                }),
                op: TypeCmp::Is,
                right: to_ast_node(TypeCmpType::Role)
            }))]
        );
    }

    #[test]
    fn single_prop_literal() {
        let token_name = to_ast_node("test");
        let prop_name = to_ast_node("prop");
        let token_decl = TokenDecl {
            name: to_ast_node("token"),
            props: to_ast_node(Arc::new([to_ast_node(TokenPropDecl {
                name: prop_name.clone(),
                types: Arc::new([to_ast_node(TokenFieldType::Literal)]),
            })])),
        };
        let result = token_decl_to_conditions(token_name.clone(), &token_decl);

        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result.unwrap(),
            vec![ExpressionTree::Leaf(to_ast_node(Comparison::PropType {
                left: to_ast_node(TokenProp {
                    token: token_name,
                    prop: prop_name
                }),
                op: TypeCmp::Is,
                right: to_ast_node(TypeCmpType::Literal)
            }))]
        );
    }

    #[test]
    fn single_prop_token() {
        let token_name = to_ast_node("test");
        let prop_name = to_ast_node("prop");
        let token_decl = TokenDecl {
            name: to_ast_node("token"),
            props: to_ast_node(Arc::new([to_ast_node(TokenPropDecl {
                name: prop_name.clone(),
                types: Arc::new([to_ast_node(TokenFieldType::Token(to_ast_node("token")))]),
            })])),
        };
        let result = token_decl_to_conditions(token_name.clone(), &token_decl);

        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result.unwrap(),
            vec![ExpressionTree::Leaf(to_ast_node(Comparison::PropType {
                left: to_ast_node(TokenProp {
                    token: token_name,
                    prop: prop_name
                }),
                op: TypeCmp::Is,
                right: to_ast_node(TypeCmpType::Token)
            }))]
        );
    }

    #[test]
    fn single_prop_lit_val() {
        let token_name = to_ast_node("test");
        let prop_name = to_ast_node("prop");
        let lit_val = to_ast_node("value");
        let token_decl = TokenDecl {
            name: to_ast_node("token"),
            props: to_ast_node(Arc::new([to_ast_node(TokenPropDecl {
                name: prop_name.clone(),
                types: Arc::new([to_ast_node(TokenFieldType::LiteralValue(lit_val.clone()))]),
            })])),
        };
        let result = token_decl_to_conditions(token_name.clone(), &token_decl);

        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result.unwrap(),
            vec![ExpressionTree::Leaf(to_ast_node(Comparison::PropLit {
                left: to_ast_node(TokenProp {
                    token: token_name,
                    prop: prop_name
                }),
                op: BoolCmp::Eq,
                right: lit_val
            }))]
        );
    }

    #[test]
    fn single_prop_union() {
        let token_name = to_ast_node("test");
        let prop_name = to_ast_node("prop");
        let lit_val = to_ast_node("value");
        let token_decl = TokenDecl {
            name: to_ast_node("token"),
            props: to_ast_node(Arc::new([to_ast_node(TokenPropDecl {
                name: prop_name.clone(),
                types: Arc::new([
                    to_ast_node(TokenFieldType::None),
                    to_ast_node(TokenFieldType::File),
                    to_ast_node(TokenFieldType::LiteralValue(lit_val.clone())),
                ]),
            })])),
        };
        let result = token_decl_to_conditions(token_name.clone(), &token_decl);

        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result.unwrap(),
            vec![ExpressionTree::Node {
                left: Box::new(ExpressionTree::Leaf(to_ast_node(Comparison::PropLit {
                    left: to_ast_node(TokenProp {
                        token: token_name.clone(),
                        prop: prop_name.clone()
                    }),
                    op: BoolCmp::Eq,
                    right: lit_val
                }))),
                op: BoolOp::Or,
                right: Box::new(ExpressionTree::Node {
                    left: Box::new(ExpressionTree::Leaf(to_ast_node(Comparison::PropType {
                        left: to_ast_node(TokenProp {
                            token: token_name.clone(),
                            prop: prop_name.clone()
                        }),
                        op: TypeCmp::Is,
                        right: to_ast_node(TypeCmpType::File)
                    }))),
                    op: BoolOp::Or,
                    right: Box::new(ExpressionTree::Leaf(to_ast_node(Comparison::PropType {
                        left: to_ast_node(TokenProp {
                            token: token_name.clone(),
                            prop: prop_name.clone()
                        }),
                        op: TypeCmp::Is,
                        right: to_ast_node(TypeCmpType::None)
                    })))
                })
            }]
        );
    }

    #[test]
    fn multiple_prop() {
        let token_name = to_ast_node("test");
        let prop_name_1 = to_ast_node("prop1");
        let prop_name_2 = to_ast_node("prop2");
        let lit_val = to_ast_node("value");
        let token_decl = TokenDecl {
            name: to_ast_node("token"),
            props: to_ast_node(Arc::new([
                to_ast_node(TokenPropDecl {
                    name: prop_name_1.clone(),
                    types: Arc::new([to_ast_node(TokenFieldType::LiteralValue(lit_val.clone()))]),
                }),
                to_ast_node(TokenPropDecl {
                    name: prop_name_2.clone(),
                    types: Arc::new([to_ast_node(TokenFieldType::File)]),
                }),
            ])),
        };
        let result = token_decl_to_conditions(token_name.clone(), &token_decl);

        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result.unwrap(),
            vec![
                ExpressionTree::Leaf(to_ast_node(Comparison::PropLit {
                    left: to_ast_node(TokenProp {
                        token: token_name.clone(),
                        prop: prop_name_1
                    }),
                    op: BoolCmp::Eq,
                    right: lit_val
                })),
                ExpressionTree::Leaf(to_ast_node(Comparison::PropType {
                    left: to_ast_node(TokenProp {
                        token: token_name.clone(),
                        prop: prop_name_2
                    }),
                    op: TypeCmp::Is,
                    right: to_ast_node(TypeCmpType::File)
                }))
            ]
        );
    }
}
