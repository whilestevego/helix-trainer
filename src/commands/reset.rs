use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::commands::verify::get_exercise_files;
use crate::display;
use crate::exercises::EXERCISES;

fn restore_from_template(file_path: &Path, exercises_dir: &Path) -> Result<bool> {
    let rel = file_path
        .strip_prefix(exercises_dir)
        .unwrap_or(file_path);

    let template = EXERCISES.get_file(rel);

    match template {
        None => {
            display::failure(&format!("No template found for {}", rel.display()));
            Ok(false)
        }
        Some(file) => {
            fs::write(file_path, file.contents())
                .with_context(|| format!("writing {}", file_path.display()))?;
            display::success(&format!("Reset {}", rel.display()));
            Ok(true)
        }
    }
}

pub fn reset_file(file_path: &Path) -> Result<()> {
    // Determine exercises dir from the file path
    let path_str = file_path.display().to_string();
    let parts: Vec<&str> = path_str.split('/').collect();
    let ex_idx = parts.iter().rposition(|&p| p == "exercises");

    match ex_idx {
        None => {
            eprintln!("File path must be inside an exercises/ directory.");
            std::process::exit(1);
        }
        Some(idx) => {
            let exercises_path = parts[..=idx].join("/");
            let exercises_dir = Path::new(&exercises_path);
            restore_from_template(file_path, exercises_dir)?;
        }
    }

    Ok(())
}

pub fn reset_all(exercises_dir: &Path) -> Result<()> {
    let files = get_exercise_files(exercises_dir)?;

    if files.is_empty() {
        println!("No .hxt exercise files found.");
        return Ok(());
    }

    let mut count = 0usize;
    for file in &files {
        if restore_from_template(file, exercises_dir)? {
            count += 1;
        }
    }

    println!("\nReset {}/{} exercises.", count, files.len());

    Ok(())
}
