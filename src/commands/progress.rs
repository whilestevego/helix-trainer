use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::commands::verify::get_exercise_files;
use crate::display;
use crate::hxt;

struct ModuleStats {
    total: usize,
    completed: usize,
}

pub fn run(exercises_dir: &Path) -> Result<()> {
    let files = get_exercise_files(exercises_dir)?;

    if files.is_empty() {
        println!("No .hxt exercise files found.");
        return Ok(());
    }

    let mut modules: BTreeMap<String, ModuleStats> = BTreeMap::new();

    for file in &files {
        let rel = file
            .strip_prefix(exercises_dir)
            .unwrap_or(file);
        let module_name = rel
            .parent()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| ".".to_string());

        let content = fs::read_to_string(file)
            .with_context(|| format!("reading {}", file.display()))?;
        let result = hxt::verify_content(&content);

        let stats = modules
            .entry(module_name)
            .or_insert(ModuleStats {
                total: 0,
                completed: 0,
            });
        stats.total += 1;
        if result.passed {
            stats.completed += 1;
        }
    }

    let mut total_completed = 0usize;
    let mut total_exercises = 0usize;

    println!("\n  HELIX TRAINER — Progress\n");
    println!("  Module                       Progress");
    println!("  ─────────────────────────────────────────");

    for (name, stats) in &modules {
        total_completed += stats.completed;
        total_exercises += stats.total;

        let pct = if stats.total > 0 {
            (stats.completed as f64 / stats.total as f64 * 100.0).round() as usize
        } else {
            0
        };

        let bar = display::progress_bar(stats.completed, stats.total, 15);

        let status = if stats.completed == stats.total {
            format!("{}", "✓".green())
        } else {
            " ".to_string()
        };

        let count = format!("{}/{}", stats.completed, stats.total);
        println!(
            "  {} {:28} {} {:>5}  {}%",
            status, name, bar, count, pct
        );
    }

    let overall_pct = if total_exercises > 0 {
        (total_completed as f64 / total_exercises as f64 * 100.0).round() as usize
    } else {
        0
    };

    println!("  ─────────────────────────────────────────");
    println!(
        "  Total: {}/{} exercises completed ({}%)\n",
        total_completed, total_exercises, overall_pct
    );

    Ok(())
}
