use std::fmt;

use crate::{compiler::CompilationStage, parser::Rule};

pub(crate) type PestError = pest::error::Error<Rule>;
pub(crate) type ErrorVariant = pest::error::ErrorVariant<Rule>;

pub struct CompilationError {
    pub(crate) stage: CompilationStage,
    pub(crate) exit_code: i32,
    pub(crate) inner: Box<dyn fmt::Display>,
}

impl fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error occurred when {}: {}", self.stage, self.inner) // user-facing output
    }
}

impl fmt::Debug for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

pub fn produce_unexpected_pair_error<R>(pair: pest::iterators::Pair<Rule>) -> Result<R, CompilationError> {
    let rule = pair.as_rule();
    let span = pair.as_span();
    let pair = pair.as_str().clone();
    let message = format!("Unexpected rule {:?} ({})", rule, pair);
    Err(CompilationError {
        stage: CompilationStage::BuildAst,
        exit_code: exitcode::DATAERR,
        inner: Box::new(PestError::new_from_span(
            pest::error::ErrorVariant::CustomError { message },
            span,
        )),
    })
}
