pub mod ast;
pub mod compiler;
pub mod errors;
pub mod parser;

mod cli;

fn main() -> ! {
    let result = cli::Cli::new().run();
    match result {
        Ok(_) => std::process::exit(exitcode::OK),
        Err(e) => {
            println!("{}", e);
            std::process::exit(e.exit_code)
        }
    }
}
