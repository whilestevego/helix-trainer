use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::exercises::EXERCISES;

fn extract_dir(dir: &include_dir::Dir<'_>, dest: &Path) -> Result<usize> {
    let mut count = 0;

    for file in dir.files() {
        let dest_path = dest.join(file.path());
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("creating directory {}", parent.display()))?;
        }
        fs::write(&dest_path, file.contents())
            .with_context(|| format!("writing {}", dest_path.display()))?;
        count += 1;
    }

    for subdir in dir.dirs() {
        count += extract_dir(subdir, dest)?;
    }

    Ok(count)
}

pub fn run(target_arg: Option<&Path>) -> Result<()> {
    let target = target_arg
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| Path::new("helix-exercises").to_path_buf());
    let target = if target.is_relative() {
        std::env::current_dir()?.join(&target)
    } else {
        target
    };

    let exercises_dest = target.join("exercises");

    // Check if target already has exercises
    if exercises_dest.join("README.md").exists() {
        eprintln!(
            "! {} already contains exercises.",
            target.display()
        );
        eprintln!("  Use 'helix-trainer reset' from that directory to restore them.");
        std::process::exit(1);
    }

    println!("\n  Generating Helix training exercises...\n");

    let count = extract_dir(&EXERCISES, &exercises_dest)?;

    // Create .gitignore
    fs::write(target.join(".gitignore"), "*.db\n.DS_Store\n")?;

    println!(
        "  ✅ Created {} exercise files in {}/exercises/",
        count,
        target.display()
    );

    let display_name = target_arg
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "helix-exercises".to_string());

    println!(
        r#"
  Next steps:
    cd {}
    helix-trainer              Launch the TUI trainer
    # Open exercise files in your editor in a split pane
"#,
        display_name
    );

    Ok(())
}
