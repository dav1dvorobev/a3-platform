use clap::{Parser, Subcommand};
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, Parser)]
#[command(name = "a3-platform", bin_name = "a3-platform")]
#[command(
    about = "A3 (Agent Actor-based Architecture) Platform", long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run node from a manifest file
    Run { path: PathBuf },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Run { path } => run(path).await,
    }
}

async fn run(path: PathBuf) -> ExitCode {
    let manifest = match a3_manifest::Manifest::from_path(path) {
        Ok(manifest) => manifest,
        Err(e) => {
            eprintln!("failed to load manifest: {e}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(e) = a3_runtime::serve(&manifest).await {
        eprintln!("failed to run node: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
