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
                    TokenFieldType::None => Some(Comparison::PropType {
                        left: token_prop_node,
                        op: TypeCmp::Is,
                        right: AstNode {
                            value: TypeCmpType::None,
                            span: field_type.span,
                        },
                    }),
                    TokenFieldType::File => Some(Comparison::PropType {
                        left: token_prop_node,
                        op: TypeCmp::Is,
                        right: AstNode {
                            value: TypeCmpType::File,
                            span: field_type.span,
                        },
                    }),
                    TokenFieldType::Role => Some(Comparison::PropType {
                        left: token_prop_node,
                        op: TypeCmp::Is,
                        right: AstNode {
                            value: TypeCmpType::Role,
                            span: field_type.span,
                        },
                    }),
                    TokenFieldType::Literal => Some(Comparison::PropType {
                        left: token_prop_node,
                        op: TypeCmp::Is,
                        right: AstNode {
                            value: TypeCmpType::Literal,
                            span: field_type.span,
                        },
                    }),
                    TokenFieldType::LiteralValue(v) => Some(Comparison::PropLit {
                        left: token_prop_node,
                        op: BoolCmp::Eq,
                        right: v.clone(),
                    }),
                    TokenFieldType::Token(_) => None,
                };

                match (leaf, acc) {
                    (None, acc) => acc,
                    (Some(leaf), None) => Some(ExpressionTree::Leaf(AstNode {
                        value: leaf,
                        span: node.span,
                    })),
                    (Some(leaf), Some(acc)) => Some(ExpressionTree::Node {
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
