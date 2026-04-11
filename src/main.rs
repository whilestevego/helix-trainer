mod commands;
mod display;
mod exercises;
mod hxt;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "helix-trainer",
    about = "Interactive Helix keybinding exercises for Zed",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate exercise project (default: ./helix-exercises)
    Init {
        /// Target directory
        dir: Option<PathBuf>,
    },
    /// Check one exercise, or all if no file given
    Verify {
        /// Exercise file to verify
        file: Option<PathBuf>,
    },
    /// Show completion stats per module
    Progress,
    /// Reset one exercise, or all if no file given
    Reset {
        /// Exercise file to reset
        file: Option<PathBuf>,
    },
    /// Show the next incomplete exercise
    Next,
}

fn find_exercises_dir() -> PathBuf {
    let cwd = std::env::current_dir().expect("cannot determine current directory");
    if cwd.ends_with("exercises") {
        cwd
    } else {
        cwd.join("exercises")
    }
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Init { dir }) => commands::init::run(dir.as_deref()),
        Some(Commands::Verify { file }) => {
            if let Some(file) = file {
                let path = if file.is_relative() {
                    std::env::current_dir().unwrap().join(&file)
                } else {
                    file
                };
                match commands::verify::verify_file(&path) {
                    Ok(passed) => std::process::exit(if passed { 0 } else { 1 }),
                    Err(e) => Err(e),
                }
            } else {
                match commands::verify::verify_all(&find_exercises_dir()) {
                    Ok(all_passed) => std::process::exit(if all_passed { 0 } else { 1 }),
                    Err(e) => Err(e),
                }
            }
        }
        Some(Commands::Progress) => commands::progress::run(&find_exercises_dir()),
        Some(Commands::Reset { file }) => {
            if let Some(file) = file {
                let path = if file.is_relative() {
                    std::env::current_dir().unwrap().join(&file)
                } else {
                    file
                };
                commands::reset::reset_file(&path)
            } else {
                commands::reset::reset_all(&find_exercises_dir())
            }
        }
        Some(Commands::Next) => commands::next::run(&find_exercises_dir()),
        None => {
            use clap::CommandFactory;
            Cli::command().print_help().ok();
            println!();
            std::process::exit(0);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
