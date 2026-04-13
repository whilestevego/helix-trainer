use std::path::PathBuf;

use clap::{Parser, Subcommand};

use helixir::{commands, find_exercises_dir, tui};

#[derive(Parser)]
#[command(
    name = "helixir",
    about = "A practice elixir for the Helix editor",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate exercise project (default: ./helixir-exercises)
    Init {
        /// Target directory
        dir: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Init { dir }) => commands::init::run(dir.as_deref()),
        None => {
            let exercises_dir = find_exercises_dir();
            if !exercises_dir.exists() {
                eprintln!("No exercises/ directory found. Run 'helixir init' first.");
                std::process::exit(1);
            }
            tui::run(exercises_dir).await
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
