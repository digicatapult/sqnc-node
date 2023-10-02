use pest::Parser;
use std::{env, fmt::Display, fs, io};

pub mod parser;
use parser::*;

pub mod ast;
pub use ast::parse_ast;
use ast::*;

#[derive(Debug)]
pub enum CompilationStage {
    ParseGrammar,
    BuildAst
}

impl Display for CompilationStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilationStage::ParseGrammar => write!(f, "parsing grammar"),
            CompilationStage::BuildAst => write!(f, "building ast")
        }
    }
}

pub fn parse_str_to_ast(input: &str) -> Result<Vec<AstNode<AstRoot>>, AstBuildError> {
    let pairs = DscpParser::parse(Rule::main, input);
    if let Err(e) = pairs {
        return Err(AstBuildError {
            stage: CompilationStage::ParseGrammar,
            inner: e
        });
    }
    let pairs = pairs.unwrap();

    parse_ast(pairs)
}

fn main() -> io::Result<()> {
    let file_path = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(file_path)?;

    let ast = parse_str_to_ast(&contents);

    if let Err(e) = ast {
        eprintln!("{}", e);
        return Ok(());
    }

    let ast = ast.unwrap();
    let token_decls = ast.iter().filter_map(|decl| match &decl.value {
        AstRoot::TokenDecl(t) => Some(&t.value),
        AstRoot::FnDecl(_) => None
    });

    let fn_decls = ast.iter().filter_map(|decl| match &decl.value {
        AstRoot::TokenDecl(_) => None,
        AstRoot::FnDecl(f) => Some(&f.value)
    });

    token_decls.for_each(|t| println!("{}\n", t));
    fn_decls.for_each(|f| println!("{}\n", f));

    Ok(())
}
