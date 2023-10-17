use std::fmt::Display;

mod parse;
pub use parse::parse_str_to_ast;

mod flatten;
pub use flatten::flatten_fns;

#[derive(Debug, PartialEq)]
pub enum CompilationStage {
    ParseGrammar,
    BuildAst,
    ReduceFns,
}

impl Display for CompilationStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilationStage::ParseGrammar => write!(f, "parsing grammar"),
            CompilationStage::BuildAst => write!(f, "building ast"),
            CompilationStage::ReduceFns => write!(f, "parsing function definitions"),
        }
    }
}
