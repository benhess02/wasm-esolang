use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Args)]
struct Run {
    file: PathBuf,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run(Run),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run(r) => {
            run(r).expect("must run");
        }
    }
}

fn run(r: Run) -> Result<(), String> {
    let _s: String = std::fs::read_to_string(r.file).map_err(|e| format!("{e}"))?;
    // TODO: pass s to the interpreter
    Ok(())
}
