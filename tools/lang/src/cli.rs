use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::{
    ast::types::AstRoot,
    compiler::{flatten_fns, parse_str_to_ast},
    errors::CompilationError,
};

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "dscp-lang", version, author)]
#[command(about = "Tool for checking and compiling dscp token specifications", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Parse {
        #[arg(help = "Path to dscp token specification file")]
        file_path: PathBuf,

        #[arg(
            short,
            long,
            help = "Output full token and function declaration",
            default_value_t = false
        )]
        verbose: bool,
    },
    #[command(arg_required_else_help = true)]
    Build {
        #[arg(help = "Path to dscp token specification file")]
        file_path: PathBuf,
        #[arg(
            short,
            long,
            help = "Output full token and function declaration",
            default_value_t = false
        )]
        verbose: bool,
    },
}

impl Cli {
    pub(crate) fn new() -> Self {
        Cli::parse()
    }

    pub(crate) fn run(&self) -> Result<(), CompilationError> {
        match &self.command {
            Commands::Parse { file_path, verbose } => {
                println!("Loading file {}", file_path.to_str().unwrap());

                let contents = fs::read_to_string(file_path).unwrap();

                let ast = parse_str_to_ast(&contents)?;

                let token_decls = ast.iter().filter_map(|decl| match &decl.value {
                    AstRoot::TokenDecl(t) => Some(&t.value),
                    AstRoot::FnDecl(_) => None,
                });

                let fn_decls = ast.iter().filter_map(|decl| match &decl.value {
                    AstRoot::TokenDecl(_) => None,
                    AstRoot::FnDecl(f) => Some(&f.value),
                });

                match verbose {
                    true => {
                        println!("\n------------------");
                        println!("Token Declarations");
                        println!("------------------");
                        println!("");
                        token_decls.for_each(|t| println!("{}\n", t));
                        println!("");
                        println!("---------------------");
                        println!("Function Declarations");
                        println!("---------------------");
                        println!("");
                        fn_decls.for_each(|f| println!("{}\n", f));
                        println!("");
                    }
                    false => {
                        println!("\n------------------");
                        println!("Token Declarations");
                        println!("------------------");
                        println!("");
                        token_decls.for_each(|t| println!("{}", t.name));
                        println!("");
                        println!("---------------------");
                        println!("Function Declarations");
                        println!("---------------------");
                        println!("");
                        fn_decls.for_each(|f| println!("{} {}", f.visibility, f.name));
                        println!("");
                    }
                }

                Ok(())
            }
            Commands::Build { file_path, verbose } => {
                println!("Loading file {}", file_path.to_str().unwrap());

                let contents = fs::read_to_string(file_path).unwrap();

                let ast = parse_str_to_ast(&contents)?;
                let ast = flatten_fns(ast)?;

                let token_decls = ast.iter().filter_map(|decl| match &decl.value {
                    AstRoot::TokenDecl(t) => Some(&t.value),
                    AstRoot::FnDecl(_) => None,
                });

                let fn_decls = ast.iter().filter_map(|decl| match &decl.value {
                    AstRoot::TokenDecl(_) => None,
                    AstRoot::FnDecl(f) => Some(&f.value),
                });

                match verbose {
                    true => {
                        println!("\n------------------");
                        println!("Token Declarations");
                        println!("------------------");
                        println!("");
                        token_decls.for_each(|t| println!("{}\n", t));
                        println!("");
                        println!("---------------------");
                        println!("Function Declarations");
                        println!("---------------------");
                        println!("");
                        fn_decls.for_each(|f| println!("{}\n", f));
                        println!("");
                    }
                    false => {
                        println!("\n------------------");
                        println!("Token Declarations");
                        println!("------------------");
                        println!("");
                        token_decls.for_each(|t| println!("{}", t.name));
                        println!("");
                        println!("---------------------");
                        println!("Function Declarations");
                        println!("---------------------");
                        println!("");
                        fn_decls.for_each(|f| println!("{} {}", f.visibility, f.name));
                        println!("");
                    }
                }

                Ok(())
            }
        }
    }
}
