use std::fmt::Display;

use pest::Parser;

use crate::{ast::parse_ast, errors::CompilationError, parser::*};

#[derive(Debug)]
pub enum CompilationStage {
    LoadFile,
    ParseGrammar,
    BuildAst,
}

impl Display for CompilationStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilationStage::ParseGrammar => write!(f, "parsing grammar"),
            CompilationStage::BuildAst => write!(f, "building ast"),
            CompilationStage::LoadFile => write!(f, "loading input"),
        }
    }
}

pub fn parse_str_to_ast(input: &str) -> Result<crate::ast::Ast, CompilationError> {
    let pairs = DscpParser::parse(Rule::main, input);
    if let Err(e) = pairs {
        return Err(CompilationError {
            stage: CompilationStage::ParseGrammar,
            exit_code: exitcode::DATAERR,
            inner: Box::new(e),
        });
    }
    let pairs = pairs.unwrap();

    parse_ast(pairs)
}

#[cfg(test)]
mod test {
    use super::parse_str_to_ast;

    #[test]
    fn valid_empty_token() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            struct TestToken {}
        "##
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn valid_token_fields() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            struct TestToken {
                role_field: Role,
                token_field: Test,
                literal_field: Literal,
                file_field: File,
                none_field: None,
                spec_literal_field: "test",
                union_field: "a" | "b" | "c"
            }
        "##
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn valid_token_fields_trailing_comma() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            struct TestToken {
                role_field: Role,
                token_field: Test,
                literal_field: Literal,
                file_field: File,
                none_field: None,
                spec_literal_field: "test",
                union_field: "a" | "b" | "c",
            }
        "##
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn invalid_token_name() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            struct Test-Token {}
        "##
            )
            .is_ok(),
            false
        );
    }

    #[test]
    fn invalid_field_name() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            struct TestToken {
                invalid-name: Role
            }
        "##
            )
            .is_ok(),
            false
        );
    }

    #[test]
    fn invalid_field_type() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            struct TestToken {
                name: Ro-le
            }
        "##
            )
            .is_ok(),
            false
        );
    }

    #[test]
    fn valid_empty_fn() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            fn Test || => || where {}
        "##
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn valid_pub_fn() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            pub fn Test || => || where {}
        "##
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn valid_priv_fn() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            priv fn Test || => || where {}
        "##
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn valid_args() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            fn Test |
                foo: Bar
            | => |
                biz: Baz
            | where {}
        "##
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn valid_args_trailing_commas() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            fn Test |
                foo: Bar,
            | => |
                biz: Baz,
            | where {}
        "##
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn valid_multiple_args() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            fn Test |
                foo: Bar,
                foo2: Bar
            | => |
                biz: Baz,
                biz2: Baz,
            | where {}
        "##
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn invalid_input_arg_name() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            fn Test |
                fo-o: Bar,
            | => || where {}
        "##
            )
            .is_ok(),
            false
        );
    }

    #[test]
    fn invalid_input_arg_type() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            fn Test |
                foo: B-ar,
            | => || where {}
        "##
            )
            .is_ok(),
            false
        );
    }

    #[test]
    fn invalid_output_arg_name() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            fn Test || => |
                fo-o: Bar,
            | where {}
        "##
            )
            .is_ok(),
            false
        );
    }

    #[test]
    fn invalid_output_arg_type() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            fn Test || => |
                foo: B-ar,
            | where {}
        "##
            )
            .is_ok(),
            false
        );
    }

    #[test]
    fn valid_where_eq() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            fn Test |
                foo: Bar,
            | => |
                biz: Baz,
            | where {
                foo.a == biz.b
            }
        "##
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn valid_where_neq() {
        assert_eq!(
            parse_str_to_ast(
                r##"
            fn Test |
                foo: Bar,
            | => |
                biz: Baz,
            | where {
                foo.a != biz.b
            }
        "##
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn valid_end_to_end() {
        let result = parse_str_to_ast(
            r##"
            struct TestToken {
                role_field: Role,
                token_field: TestToken,
                literal_field: Literal,
                file_field: File,
                none_field: None,
                spec_literal_field: "test",
                union_field: "a" | "b" | "c",
            }

            pub fn TestFn | in: TestToken | => | out: TestToken | where {
                out.role_field == in.role_field,
                out.literal_field == "literal",
                (in.union_field == "a" | out.union_field == "b"),
            }
        "##,
        );
        assert_eq!(result.is_ok(), true);
    }
}
