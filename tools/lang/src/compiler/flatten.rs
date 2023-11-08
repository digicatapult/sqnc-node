use std::{
    collections::{HashMap, HashSet},
    ops::ControlFlow,
    sync::Arc,
};

use crate::{
    ast::types::*,
    errors::{CompilationError, ErrorVariant, PestError},
};

use super::CompilationStage;

fn transform_name<'a>(
    name: AstNode<'a, &'a str>,
    token_name_transforms: Arc<HashMap<&'a str, &'a str>>,
) -> Result<AstNode<'a, &'a str>, CompilationError> {
    Ok(AstNode {
        value: token_name_transforms.get(name.value).ok_or(CompilationError {
            stage: CompilationStage::ReduceFns,
            exit_code: exitcode::DATAERR,
            inner: PestError::new_from_span(
                ErrorVariant::CustomError {
                    message: "Undeclared token".into(),
                },
                name.span,
            ),
        })?,
        span: name.span,
    })
}

fn transform_comparison<'a>(
    comparison: Comparison<'a>,
    token_name_transforms: Arc<HashMap<&'a str, &'a str>>,
) -> Result<Comparison<'a>, CompilationError> {
    match comparison {
        Comparison::Fn { name, inputs, outputs } => Ok(Comparison::Fn {
            name,
            inputs: AstNode {
                value: inputs
                    .value
                    .into_iter()
                    .map(|i| transform_name(i.clone(), token_name_transforms.clone()))
                    .collect::<Result<_, _>>()?,
                span: inputs.span,
            },
            outputs: AstNode {
                value: outputs
                    .value
                    .into_iter()
                    .map(|o| transform_name(o.clone(), token_name_transforms.clone()))
                    .collect::<Result<_, _>>()?,
                span: outputs.span,
            },
        }),
        Comparison::PropLit { left, op, right } => Ok(Comparison::PropLit {
            left: AstNode {
                value: TokenProp {
                    token: transform_name(left.value.token, token_name_transforms.clone())?,
                    prop: left.value.prop,
                },
                span: left.span,
            },
            op,
            right,
        }),
        Comparison::PropSender { left, op } => Ok(Comparison::PropSender {
            left: AstNode {
                value: TokenProp {
                    token: transform_name(left.value.token, token_name_transforms.clone())?,
                    prop: left.value.prop,
                },
                span: left.span,
            },
            op,
        }),
        Comparison::TokenToken { left, op, right } => Ok(Comparison::TokenToken {
            left: transform_name(left, token_name_transforms.clone())?,
            op,
            right: transform_name(right, token_name_transforms.clone())?,
        }),
        Comparison::PropToken { left, op, right } => Ok(Comparison::PropToken {
            left: AstNode {
                value: TokenProp {
                    token: transform_name(left.value.token, token_name_transforms.clone())?,
                    prop: left.value.prop,
                },
                span: left.span,
            },
            op,
            right: transform_name(right, token_name_transforms.clone())?,
        }),
        Comparison::PropProp { left, op, right } => Ok(Comparison::PropProp {
            left: AstNode {
                value: TokenProp {
                    token: transform_name(left.value.token, token_name_transforms.clone())?,
                    prop: left.value.prop,
                },
                span: left.span,
            },
            op,
            right: AstNode {
                value: TokenProp {
                    token: transform_name(right.value.token, token_name_transforms.clone())?,
                    prop: right.value.prop,
                },
                span: right.span,
            },
        }),
        Comparison::PropType { left, op, right } => Ok(Comparison::PropType {
            left: AstNode {
                value: TokenProp {
                    token: transform_name(left.value.token, token_name_transforms.clone())?,
                    prop: left.value.prop,
                },
                span: left.span,
            },
            op,
            right,
        }),
    }
}

fn transform_expression<'a>(
    expr: ExpressionTree<'a>,
    token_name_transforms: Arc<HashMap<&'a str, &'a str>>,
) -> Result<ExpressionTree<'a>, CompilationError> {
    match expr {
        ExpressionTree::Leaf(c) => Ok(ExpressionTree::Leaf(AstNode {
            value: transform_comparison(c.value, token_name_transforms.clone())?,
            span: c.span,
        })),
        ExpressionTree::Not(e) => Ok(ExpressionTree::Not(Box::new(transform_expression(
            *e,
            token_name_transforms,
        )?))),
        ExpressionTree::Node { left, op, right } => Ok(ExpressionTree::Node {
            left: Box::new(transform_expression(*left, token_name_transforms.clone())?),
            op,
            right: Box::new(transform_expression(*right, token_name_transforms.clone())?),
        }),
    }
}

fn transform_conditions<'a, 'b>(
    conditions: Vec<ExpressionTree<'a>>,
    token_name_transforms: HashMap<&'a str, &'a str>,
) -> Result<Vec<ExpressionTree<'a>>, CompilationError> {
    let token_name_transforms = Arc::new(token_name_transforms);
    conditions
        .into_iter()
        .map(|expr| transform_expression(expr, token_name_transforms.clone()))
        .collect::<Result<Vec<_>, _>>()
}

fn check_args<'a>(
    decl: &AstNode<'a, Arc<[AstNode<'a, FnArg<'a>>]>>,
    call: &AstNode<'a, Arc<[AstNode<'a, FnArg<'a>>]>>,
) -> Result<(), CompilationError> {
    if decl.value.len() != call.value.len() {
        return Err(CompilationError {
            stage: CompilationStage::ReduceFns,
            exit_code: exitcode::DATAERR,
            inner: PestError::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("Expected {} arguments got {}", decl.value.len(), call.value.len()).into(),
                },
                call.span,
            ),
        });
    }

    decl.value
        .iter()
        .zip(call.value.iter())
        .map(|(a, b)| match a.value.token_type.value == b.value.token_type.value {
            true => Ok(()),
            false => Err(CompilationError {
                stage: CompilationStage::ReduceFns,
                exit_code: exitcode::DATAERR,
                inner: PestError::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!(
                            "Expected argument of type {} got {}",
                            a.value.token_type.value, b.value.token_type.value
                        ),
                    },
                    b.span,
                ),
            }),
        })
        .collect::<Result<_, _>>()
}

fn lookup_args<'a>(
    decl: AstNode<'a, Arc<[AstNode<'a, FnArg<'a>>]>>,
    call: AstNode<'a, Arc<[AstNode<'a, &str>]>>,
) -> Result<AstNode<'a, Arc<[AstNode<'a, FnArg<'a>>]>>, CompilationError> {
    Ok(AstNode {
        value: call
            .value
            .clone()
            .into_iter()
            .map(|i| {
                decl.value
                    .clone()
                    .into_iter()
                    .find_map(|p| match p.value.name.value == i.value {
                        true => Some(AstNode {
                            value: p.value.clone(),
                            span: i.span,
                        }),
                        false => None,
                    })
                    .ok_or(CompilationError {
                        stage: CompilationStage::ReduceFns,
                        exit_code: exitcode::DATAERR,
                        inner: PestError::new_from_span(
                            ErrorVariant::CustomError {
                                message: "Undeclared token".into(),
                            },
                            i.span,
                        ),
                    })
            })
            .collect::<Result<_, _>>()?,
        span: call.span,
    })
}

fn flatten_expr<'a, I>(expr: I) -> ExpressionTree<'a>
where
    I: IntoIterator<Item = ExpressionTree<'a>>,
{
    let mut expr = expr.into_iter();
    let left = expr.next().unwrap();
    let right = expr.collect::<Vec<_>>();

    match right.is_empty() {
        false => ExpressionTree::Node {
            left: Box::new(left),
            op: BoolOp::And,
            right: Box::new(flatten_expr(right)),
        },
        true => left,
    }
}

fn flatten_expr_fn<'a>(
    name: AstNode<'a, &str>,
    inputs: AstNode<'a, Arc<[AstNode<'a, &str>]>>,
    outputs: AstNode<'a, Arc<[AstNode<'a, &str>]>>,
    input_decls: AstNode<'a, Arc<[AstNode<'a, FnArg<'a>>]>>,
    output_decls: AstNode<'a, Arc<[AstNode<'a, FnArg<'a>>]>>,
    called_fns: HashSet<&str>,
    all_fns: Arc<[FnDecl<'a>]>,
) -> Result<Vec<ExpressionTree<'a>>, CompilationError> {
    if called_fns.contains(name.value) {
        return Err(CompilationError {
            stage: CompilationStage::ReduceFns,
            exit_code: exitcode::DATAERR,
            inner: PestError::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("Recursive function call detected in function {}", name.value).into(),
                },
                name.span,
            ),
        });
    }
    let mut called_fns = called_fns.clone();
    called_fns.insert(name.value);

    let fn_decl = all_fns.iter().find(|f| f.name.value == name.value);

    match fn_decl {
        None => Err(CompilationError {
            stage: CompilationStage::ReduceFns,
            exit_code: exitcode::DATAERR,
            inner: PestError::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("Unknown function {}", name.value).into(),
                },
                name.span,
            ),
        }),

        Some(fn_decl) => {
            let inputs = lookup_args(input_decls.clone(), inputs)?;
            let outputs = lookup_args(output_decls.clone(), outputs)?;

            check_args(&fn_decl.inputs, &inputs)?;
            check_args(&fn_decl.outputs, &outputs)?;

            let name_map = fn_decl
                .inputs
                .value
                .iter()
                .chain(fn_decl.outputs.value.iter())
                .zip(inputs.value.iter().chain(outputs.value.iter()))
                .map(|(a, b)| (a.value.name.value.clone(), b.value.name.value.clone()))
                .collect::<HashMap<_, _>>();

            let conditions = transform_conditions(fn_decl.conditions.value.clone(), name_map)?;
            Ok(conditions
                .into_iter()
                .map(|expr| {
                    flatten_expr_fns(
                        expr,
                        input_decls.clone(),
                        output_decls.clone(),
                        called_fns.clone(),
                        all_fns.clone(),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect())
        }
    }
}

fn flatten_expr_fns<'a>(
    expr: ExpressionTree<'a>,
    input_decls: AstNode<'a, Arc<[AstNode<'a, FnArg<'a>>]>>,
    output_decls: AstNode<'a, Arc<[AstNode<'a, FnArg<'a>>]>>,
    called_fns: HashSet<&str>,
    all_fns: Arc<[FnDecl<'a>]>,
) -> Result<Vec<ExpressionTree<'a>>, CompilationError> {
    match expr {
        ExpressionTree::Not(_) => Ok(vec![expr]),
        ExpressionTree::Leaf(comp) => match comp.value {
            Comparison::Fn { name, inputs, outputs } => {
                flatten_expr_fn(name, inputs, outputs, input_decls, output_decls, called_fns, all_fns)
            }
            c => Ok(vec![ExpressionTree::Leaf(AstNode {
                value: c,
                span: comp.span,
            })]),
        },
        ExpressionTree::Node { left, op, right } => {
            let node = ExpressionTree::Node {
                left: Box::new(flatten_expr(flatten_expr_fns(
                    *left.clone(),
                    input_decls.clone(),
                    output_decls.clone(),
                    called_fns.clone(),
                    all_fns.clone(),
                )?)),
                op: op.clone(),
                right: Box::new(flatten_expr(flatten_expr_fns(
                    *right.clone(),
                    input_decls.clone(),
                    output_decls.clone(),
                    called_fns.clone(),
                    all_fns.clone(),
                )?)),
            };
            Ok(vec![node])
        }
    }
}

fn flatten_fn<'a>(fn_decl: FnDecl<'a>, all_fns: Arc<[FnDecl<'a>]>) -> Result<FnDecl<'a>, CompilationError> {
    let mut called_fns: HashSet<&str> = HashSet::new();
    called_fns.insert(fn_decl.name.value);

    Ok(FnDecl {
        visibility: fn_decl.visibility,
        name: fn_decl.name,
        inputs: fn_decl.inputs.clone(),
        outputs: fn_decl.outputs.clone(),
        conditions: AstNode {
            span: fn_decl.conditions.span,
            value: fn_decl
                .conditions
                .value
                .into_iter()
                .map(|expr| {
                    flatten_expr_fns(
                        expr,
                        fn_decl.inputs.clone(),
                        fn_decl.outputs.clone(),
                        called_fns.clone(),
                        all_fns.clone(),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect(),
        },
    })
}

fn check_fn_decl_duplicate_args(fn_decl: &FnDecl) -> Result<(), CompilationError> {
    let FnDecl { inputs, outputs, .. } = fn_decl;

    let mut all_names = inputs
        .value
        .into_iter()
        .chain(outputs.value.into_iter())
        .map(|arg| arg.value.name.clone())
        .collect::<Vec<_>>();

    // if there are no arguments then we're fine
    if all_names.len() == 0 {
        return Ok(());
    }

    // sort names so we can check for duplicates
    all_names.sort_by(|a, b| a.value.partial_cmp(b.value).unwrap());

    let mut all_names = all_names.into_iter();
    let first = all_names.next().unwrap();
    // try to find a duplicate by folding over. A duplicate will have the previous name in the list equal to the current one
    let try_find_duplicate = all_names.try_fold(first, |prev, arg| match prev.value == arg.value {
        true => ControlFlow::Break(CompilationError {
            stage: CompilationStage::BuildAst,
            exit_code: exitcode::DATAERR,
            inner: PestError::new_from_span(
                ErrorVariant::CustomError {
                    message: "Duplicate argument name".into(),
                },
                arg.span,
            ),
        }),
        false => ControlFlow::Continue(arg),
    });

    // if a break didn't occur all names were unique
    // a break means a duplicate was found
    match try_find_duplicate {
        ControlFlow::Continue(_) => Ok(()),
        ControlFlow::Break(e) => Err(e),
    }
}

fn check_fn_decls_duplicate_args(ast: &Ast) -> Result<(), CompilationError> {
    ast.clone()
        .into_iter()
        .filter_map(|f| match f.value {
            AstRoot::TokenDecl(_) => None,
            AstRoot::FnDecl(f) => Some(f.value),
        })
        .map(|f| check_fn_decl_duplicate_args(&f))
        .collect()
}

pub fn flatten_fns(ast: Ast) -> Result<Ast, CompilationError> {
    check_fn_decls_duplicate_args(&ast)?;

    let fns: Arc<_> = ast
        .clone()
        .into_iter()
        .filter_map(|f| match f.value {
            AstRoot::TokenDecl(_) => None,
            AstRoot::FnDecl(f) => Some(f.value),
        })
        .collect();

    ast.clone()
        .into_iter()
        .filter(|node| match &node.value {
            AstRoot::TokenDecl(_) => true,
            AstRoot::FnDecl(f) => f.value.visibility.value == FnVis::Public,
        })
        .map(|node| match &node.value {
            AstRoot::TokenDecl(_) => Ok(node),
            AstRoot::FnDecl(f) => Ok(AstNode {
                value: AstRoot::FnDecl(AstNode {
                    span: f.span,
                    value: flatten_fn(f.value.clone(), fns.clone())?,
                }),
                span: node.span,
            }),
        })
        .collect::<Result<Vec<_>, _>>()
}

#[cfg(test)]
mod tests {
    use super::flatten_fns;
    use crate::ast::parse_str_to_ast;

    #[test]
    fn duplicate_input_arg_decl() {
        let ast = parse_str_to_ast(
            r#"
        pub fn test | a: Foo, a: Foo | => || where {}
        "#,
        )
        .unwrap();

        let result = flatten_fns(ast);
        assert_eq!(result.is_ok(), false);
        assert_eq!(result.err().unwrap().inner.variant.message(), "Duplicate argument name");
    }

    #[test]
    fn duplicate_output_arg_decl() {
        let ast = parse_str_to_ast(
            r#"
        pub fn test || => | a: Foo, a: Foo | where {}
        "#,
        )
        .unwrap();

        let result = flatten_fns(ast);
        assert_eq!(result.is_ok(), false);
        assert_eq!(result.err().unwrap().inner.variant.message(), "Duplicate argument name");
    }

    #[test]
    fn duplicate_input_output_arg_decl() {
        let ast = parse_str_to_ast(
            r#"
        pub fn test | a: Foo | => | a: Foo | where {}
        "#,
        )
        .unwrap();

        let result = flatten_fns(ast);
        assert_eq!(result.is_ok(), false);
        assert_eq!(result.err().unwrap().inner.variant.message(), "Duplicate argument name");
    }

    #[test]
    fn recursive_direct_arg_decl() {
        let ast = parse_str_to_ast(
            r#"
        pub fn test || => || where {
            test || => ||
        }
        "#,
        )
        .unwrap();

        let result = flatten_fns(ast);
        assert_eq!(result.is_ok(), false);
        assert_eq!(
            result.err().unwrap().inner.variant.message(),
            "Recursive function call detected in function test"
        );
    }

    #[test]
    fn recursive_indirect_arg_decl() {
        let ast = parse_str_to_ast(
            r#"
        pub fn middle || => || where {
            test || => ||
        }

        pub fn test || => || where {
            middle || => ||
        }
        "#,
        )
        .unwrap();

        let result = flatten_fns(ast);
        assert_eq!(result.is_ok(), false);
        assert_eq!(
            result.err().unwrap().inner.variant.message(),
            "Recursive function call detected in function middle"
        );
    }

    #[test]
    fn undeclared_token_in_arg() {
        let ast = parse_str_to_ast(
            r#"
        pub fn middle | a: Foo | => || where {}

        pub fn test || => || where {
            middle | foo | => ||
        }
        "#,
        )
        .unwrap();

        let result = flatten_fns(ast);
        assert_eq!(result.is_ok(), false);
        assert_eq!(result.err().unwrap().inner.variant.message(), "Undeclared token");
    }

    #[test]
    fn undeclared_token_in_fn() {
        let ast = parse_str_to_ast(
            r#"
        pub fn middle || => || where {
            a.foo: File
        }

        pub fn test || => || where {
            middle || => ||
        }
        "#,
        )
        .unwrap();

        let result = flatten_fns(ast);
        assert_eq!(result.is_ok(), false);
        assert_eq!(result.err().unwrap().inner.variant.message(), "Undeclared token");
    }

    #[test]
    fn incorrect_input_arg_count() {
        let ast = parse_str_to_ast(
            r#"
        pub fn middle | a: Foo, b: Foo | => || where {}

        pub fn test | a: Foo | => || where {
            middle | a | => ||
        }
        "#,
        )
        .unwrap();

        let result = flatten_fns(ast);
        assert_eq!(result.is_ok(), false);
        assert_eq!(
            result.err().unwrap().inner.variant.message(),
            "Expected 2 arguments got 1"
        );
    }

    #[test]
    fn incorrect_output_arg_count() {
        let ast = parse_str_to_ast(
            r#"
        pub fn middle || => | a: Foo, b: Foo | where {}

        pub fn test || => | a: Foo | where {
            middle || => | a |
        }
        "#,
        )
        .unwrap();

        let result = flatten_fns(ast);
        assert_eq!(result.is_ok(), false);
        assert_eq!(
            result.err().unwrap().inner.variant.message(),
            "Expected 2 arguments got 1"
        );
    }

    #[test]
    fn incorrect_input_arg_type() {
        let ast = parse_str_to_ast(
            r#"
        pub fn middle | a: Bar | => || where {}

        pub fn test | a: Foo | => || where {
            middle | a | => ||
        }
        "#,
        )
        .unwrap();

        let result = flatten_fns(ast);
        assert_eq!(result.is_ok(), false);
        assert_eq!(
            result.err().unwrap().inner.variant.message(),
            "Expected argument of type Bar got Foo"
        );
    }

    #[test]
    fn incorrect_output_arg_type() {
        let ast = parse_str_to_ast(
            r#"
        pub fn middle || => | a: Bar | where {}

        pub fn test || => | a: Foo | where {
            middle || => | a |
        }
        "#,
        )
        .unwrap();

        let result = flatten_fns(ast);
        assert_eq!(result.is_ok(), false);
        assert_eq!(
            result.err().unwrap().inner.variant.message(),
            "Expected argument of type Bar got Foo"
        );
    }

    #[test]
    fn no_flatten() {
        let ast = parse_str_to_ast(
            r#"
        pub fn test | a: Foo | => | b: Foo | where {
            a == b,
        }
        "#,
        )
        .unwrap();

        let result = flatten_fns(ast.clone());
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), ast);
    }

    #[test]
    fn single_flatten() {
        let ast = parse_str_to_ast(
            r#"
        fn middle | a: Foo | => | b: Foo | where {
            a == b,
            a.bar == b.bar,
        }

        pub fn test | a: Foo | => | b: Foo | where {
            middle | a | => | b |,
        }
        "#,
        )
        .unwrap();

        let result = flatten_fns(ast.clone());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.len(), 1);
        let result = match result.into_iter().next().unwrap().value {
            crate::ast::types::AstRoot::TokenDecl(_) => panic!("Unexpected token decl"),
            crate::ast::types::AstRoot::FnDecl(f) => f.value,
        };
        assert_eq!(result.name.value, "test");
        assert_eq!(result.conditions.value.len(), 2);
    }

    #[test]
    fn nested_flatten() {
        let ast = parse_str_to_ast(
            r#"
        fn deep | a: Foo | => | b: Foo | where {
            a.bar == b.bar,
        }

        fn middle | a: Foo | => | b: Foo | where {
            deep | a | => | b |,
            a.bar == b.bar,
        }

        pub fn test | a: Foo | => | b: Foo | where {
            middle | a | => | b |,
        }
        "#,
        )
        .unwrap();

        let result = flatten_fns(ast.clone());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.len(), 1);
        let result = match result.into_iter().next().unwrap().value {
            crate::ast::types::AstRoot::TokenDecl(_) => panic!("Unexpected token decl"),
            crate::ast::types::AstRoot::FnDecl(f) => f.value,
        };
        assert_eq!(result.name.value, "test");
        assert_eq!(result.conditions.value.len(), 2);
    }
}
