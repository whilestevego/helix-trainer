use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::commands::verify::get_exercise_files;
use crate::hxt;

pub fn run(exercises_dir: &Path) -> Result<()> {
    let files = get_exercise_files(exercises_dir)?;
    let cwd = std::env::current_dir()?;

    for file in &files {
        let content = fs::read_to_string(file)
            .with_context(|| format!("reading {}", file.display()))?;
        let result = hxt::verify_content(&content);
        if !result.passed {
            let rel = file.strip_prefix(&cwd).unwrap_or(file);
            println!("{}", rel.display());
            return Ok(());
        }
    }

    println!("All exercises completed!");
    Ok(())
}
