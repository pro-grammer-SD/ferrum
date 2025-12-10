use clap::{Parser, Subcommand};
use anyhow::Result;

use ferrum::interpreter;

#[derive(Parser)]
#[command(author, version, about = "Ferrum - a fast, modular, Python-like language with native Rust performance and built-in GUI + CV tooling", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a .fm script
    Run { file: String },
    /// Start the Ferrum REPL
    Repl,
    /// Build (serialize) a .fm file to bytecode (prototype)
    Build { file: String },
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => {
            // Blocking run is fine for now; runs on tokio runtime.
            interpreter::run_file(&file)?;
        }
        Commands::Repl => {
            interpreter::repl()?;
        }
        Commands::Build { file } => {
            interpreter::build(&file)?;
        }
    }

    Ok(())
}
