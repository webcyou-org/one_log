use clap::Parser;
use one_log::Cli;
use env_logger;

fn main() {
    env_logger::init();

    let args = Cli::parse();
    let _ = one_log::run_from_args(args);
}
