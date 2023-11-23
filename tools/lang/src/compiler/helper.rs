use crate::{
    ast::types::AstNode,
    errors::{CompilationError, CompilationStage, PestError},
};

pub fn to_bounded_vec<I, O, V>(collection: AstNode<I>) -> Result<O, CompilationError>
where
    I: IntoIterator<Item = V>,
    O: TryFrom<Vec<V>>,
{
    let foo = collection.value.into_iter().collect::<Vec<_>>();
    let foo_len = foo.len();
    <O as TryFrom<Vec<V>>>::try_from(foo).map_err(|_| CompilationError {
        stage: CompilationStage::LengthValidation,
        exit_code: exitcode::DATAERR,
        inner: PestError::new_from_span(
            pest::error::ErrorVariant::CustomError {
                message: format!("too long or compiles to too many elements ({})", foo_len),
            },
            collection.span,
        ),
    })
}

#[cfg(test)]
pub mod tests {
    use super::to_bounded_vec;
    use crate::ast::types::AstNode;
    use dscp_runtime_types::ProcessIdentifier;

    pub fn to_ast_node<'a, V>(value: V) -> AstNode<'a, V> {
        let span = pest::Span::new("", 0, 0).unwrap();
        AstNode { value, span }
    }

    #[test]
    fn happy_path() {
        let result: Result<ProcessIdentifier, _> = to_bounded_vec(to_ast_node("small".as_bytes().to_vec()));

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            ProcessIdentifier::try_from("small".as_bytes().to_vec()).unwrap()
        );
    }

    #[test]
    fn error_too_long() {
        let result: Result<ProcessIdentifier, _> = to_bounded_vec(to_ast_node(vec![0x61u8; 33]));
        assert!(result.is_err());
    }
}
