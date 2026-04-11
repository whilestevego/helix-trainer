use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::display;
use crate::hxt;

pub fn get_exercise_files(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    use walkdir::WalkDir;

    let mut files: Vec<std::path::PathBuf> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "hxt")
        })
        .map(|e| e.into_path())
        .collect();
    files.sort();
    Ok(files)
}

pub fn verify_file(path: &Path) -> Result<bool> {
    let content =
        fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let result = hxt::verify_content(&content);

    if result.passed {
        println!("{} {}", "✓".green(), path.display());
    } else {
        println!("{} {}", "✗".red(), path.display());
        if !result.diff.is_empty() {
            println!("\n  Differences:");
            for d in &result.diff {
                println!("    line {}:", d.line_num);
                println!("      got:      \"{}\"", d.got);
                println!("      expected: \"{}\"", d.expected);
            }
        }
    }

    Ok(result.passed)
}

pub fn verify_all(exercises_dir: &Path) -> Result<bool> {
    let files = get_exercise_files(exercises_dir)?;

    if files.is_empty() {
        println!("No .hxt exercise files found.");
        return Ok(true);
    }

    let mut passed = 0usize;
    let mut failed = 0usize;

    for file in &files {
        let content = fs::read_to_string(file)
            .with_context(|| format!("reading {}", file.display()))?;
        let result = hxt::verify_content(&content);
        let rel = file
            .strip_prefix(exercises_dir)
            .unwrap_or(file)
            .display();

        if result.passed {
            display::success(&rel.to_string());
            passed += 1;
        } else {
            display::failure(&rel.to_string());
            failed += 1;
        }
    }

    println!("\n  {} passed, {} remaining\n", passed, failed);
    Ok(failed == 0)
}
