pub mod commands;
pub mod exercises;
pub mod hxt;
pub mod metadata;
pub mod progress;
pub mod tui;

use std::path::PathBuf;

pub fn find_exercises_dir() -> PathBuf {
    let cwd = std::env::current_dir().expect("cannot determine current directory");
    if cwd.ends_with("exercises") {
        cwd
    } else {
        cwd.join("exercises")
    }
}
