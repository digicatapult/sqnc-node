use std::fmt;

use crate::{parser::Rule, CompilationStage};

pub(crate) type PestError = pest::error::Error<Rule>;
pub(crate) type ErrorVariant = pest::error::ErrorVariant<Rule>;

pub struct AstBuildError {
    pub(crate) stage: CompilationStage,
    pub(crate) inner: PestError
}

impl fmt::Display for AstBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error occurred when {}: {}", self.stage, self.inner) // user-facing output
    }
}

impl fmt::Debug for AstBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

pub fn produce_unexpected_pair_error<R>(pair: pest::iterators::Pair<Rule>) -> Result<R, AstBuildError> {
    let rule = pair.as_rule();
    let span = pair.as_span();
    let pair = pair.as_str().clone();
    let message = format!("Unexpected rule {:?} ({})", rule, pair);
    Err(AstBuildError {
        stage: CompilationStage::BuildAst,
        inner: PestError::new_from_span(pest::error::ErrorVariant::CustomError { message }, span)
    })
}
