use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::{
    ast::{parse_str_to_ast, types::AstRoot},
    compiler::compile_ast_to_restrictions,
    convert::transform_to_json,
    errors::CompilationError,
};

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "dscp-lang", version, author)]
#[command(about = "Tool for checking and compiling dscp token specifications", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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

        #[arg(short, long, help = "Path of JSON file to output programs to")]
        output_file: Option<PathBuf>,

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
    pub fn new() -> Self {
        Cli::parse()
    }

    pub fn run(&self) -> Result<(), CompilationError> {
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
            Commands::Build {
                file_path,
                verbose,
                output_file,
                ..
            } => {
                println!("Loading file {}", file_path.to_str().unwrap());
                let contents = fs::read_to_string(file_path).unwrap();
                let ast = parse_str_to_ast(&contents)?;
                let programs = compile_ast_to_restrictions(ast)?;

                println!("Successfully compiled the following programs:");
                for program in &programs {
                    let program_name = String::from_utf8(program.name.to_vec()).unwrap();
                    if *verbose {
                        let program_str = transform_to_json(program, false).unwrap();
                        println!("\n{}:\n{}", program_name, program_str);
                    } else {
                        println!("\t{}", program_name);
                    }
                }

                if let Some(path) = output_file {
                    fs::write(path, transform_to_json(&programs, true).unwrap()).unwrap()
                }

                Ok(())
            }
        }
    }
}
