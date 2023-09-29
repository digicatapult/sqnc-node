use pest::{Parser, Span};
use std::{env, fmt, fs, io};

pub mod parser;
use parser::*;

pub mod ast;
use ast::*;

fn main() -> io::Result<()> {
    let file_path = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(file_path)?;

    let pairs = DscpParser::parse(Rule::main, &contents);
    if let Err(e) = pairs {
        eprintln!("Parse failed: {:?}", e);
        return Ok(());
    }
    let pairs = pairs.unwrap();

    let ast = parse_ast(pairs);

    match ast {
        Ok(ast) => println!("{:?}", ast),
        Err(e) => eprintln!("Error occurred {}", e)
    };

    Ok(())
}
