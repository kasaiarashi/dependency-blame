use clap::Parser;
use dependency_blame::cli::args::Cli;
use dependency_blame::cli::commands;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = commands::execute(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
