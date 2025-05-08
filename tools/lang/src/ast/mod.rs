use pest::Parser;

pub mod parse;
pub mod types;

use crate::{
    errors::{CompilationError, CompilationStage},
    parser::*,
};

pub use parse::parse_ast;
pub use types::Ast;

pub fn parse_str_to_ast(input: &str) -> Result<Ast, CompilationError> {
    let pairs = SqncParser::parse(Rule::main, input);
    if let Err(e) = pairs {
        return Err(CompilationError {
            stage: CompilationStage::ParseGrammar,
            exit_code: exitcode::DATAERR,
            inner: e,
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
          token TestToken {}
      "##
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn valid_token_version_attr() {
        assert_eq!(
            parse_str_to_ast(
                r##"
          [#version(42)]
          token TestToken {}
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
          token TestToken {
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
          token TestToken {
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
          token Test-Token {}
      "##
            )
            .is_ok(),
            false
        );
    }

    #[test]
    fn invalid_token_version_attr_neg() {
        assert_eq!(
            parse_str_to_ast(
                r##"
          [#version(-42)]
          token TestToken {}
      "##
            )
            .is_ok(),
            false
        );
    }

    #[test]
    fn invalid_token_version_attr_float() {
        assert_eq!(
            parse_str_to_ast(
                r##"
          [#version(4.2)]
          token TestToken {}
      "##
            )
            .is_ok(),
            false
        );
    }

    #[test]
    fn invalid_token_version_attr_alpha() {
        assert_eq!(
            parse_str_to_ast(
                r##"
          [#version(forty-two)]
          token TestToken {}
      "##
            )
            .is_ok(),
            false
        );
    }

    #[test]
    fn invalid_token_name_keyword() {
        assert_eq!(
            parse_str_to_ast(
                r##"
          token token {}
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
          token TestToken {
              invalid-name: Role
          }
      "##
            )
            .is_ok(),
            false
        );
    }

    #[test]
    fn invalid_field_name_keyword() {
        assert_eq!(
            parse_str_to_ast(
                r##"
          token TestToken {
              where: Role
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
          token TestToken {
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
    fn valid_ref_args() {
        assert_eq!(
            parse_str_to_ast(
                r##"
          fn Test |
              foo: &Bar
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
              foo2: Bar,
              foo3: &Bar,
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
    fn invalid_output_arg_ref() {
        assert_eq!(
            parse_str_to_ast(
                r##"
          fn Test || => |
              foo: &Bar,
          | where {}
      "##
            )
            .is_ok(),
            false
        );
    }

    #[test]
    fn valid_where_eq_prop_token() {
        assert_eq!(
            parse_str_to_ast(
                r##"
          fn Test |
              foo: Bar,
          | => |
              biz: Baz,
          | where {
              foo.a == biz
          }
      "##
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn valid_where_eq_token_token() {
        assert_eq!(
            parse_str_to_ast(
                r##"
          fn Test |
              foo: Bar,
          | => |
              biz: Baz,
          | where {
              foo == biz
          }
      "##
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn valid_where_eq_prop_prop() {
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
    fn valid_where_eq_prop_sender() {
        assert_eq!(
            parse_str_to_ast(
                r##"
          fn Test |
              foo: Bar,
          | => || where {
              foo.a == sender
          }
      "##
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn valid_where_eq_sender_root() {
        assert_eq!(
            parse_str_to_ast(
                r##"
          fn Test |
              foo: Bar,
          | => || where {
              sender == root
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
    fn valid_where_eq_prop_is() {
        assert_eq!(
            parse_str_to_ast(
                r##"
          fn Test |
              foo: Bar,
          | => |
              biz: Baz,
          | where {
              foo.b: File
          }
      "##
            )
            .is_ok(),
            true,
        );
    }

    #[test]
    fn valid_where_eq_prop_isnt() {
        assert_eq!(
            parse_str_to_ast(
                r##"
          fn Test |
              foo: Bar,
          | => |
              biz: Baz,
          | where {
              foo.b !: File
          }
      "##
            )
            .is_ok(),
            true,
        );
    }

    #[test]
    fn valid_end_to_end() {
        let result = parse_str_to_ast(
            r##"
          token TestToken {
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
        assert!(result.is_ok());
    }
}
