use std::fmt;

use pest::Span;

use crate::parser::Rule;

pub struct AstBuildError<'a> {
    pub(crate) message: String,
    pub(crate) span: Span<'a>
}

impl<'a> fmt::Display for AstBuildError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl<'a> fmt::Debug for AstBuildError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

pub fn produce_unexpected_pair_error<R>(pair: pest::iterators::Pair<Rule>) -> Result<R, AstBuildError> {
    let span = pair.as_span();
    let pair = pair.as_str().clone();
    let message = format!("Unexpected rule {}", pair);
    Err(AstBuildError { message, span })
}
