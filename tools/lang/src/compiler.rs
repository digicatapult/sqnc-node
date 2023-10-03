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
