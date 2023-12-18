mod benchmarking;
mod chain_spec;
mod cli;
mod command;
mod rpc;
mod service;
mod test_service;

fn main() -> sc_cli::Result<()> {
    command::run()
}
