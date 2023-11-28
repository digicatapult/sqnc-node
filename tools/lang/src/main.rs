mod ast;
mod cli;
mod compiler;
mod convert;
mod errors;
mod parser;

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
