use owo_colors::OwoColorize;

pub fn success(path: &str) {
    println!("  {} {}", "✓".green(), path);
}

pub fn failure(path: &str) {
    println!("  {} {}", "✗".red(), path);
}

pub fn warning(msg: &str) {
    println!("{} {}", "!".yellow(), msg);
}

pub fn progress_bar(completed: usize, total: usize, width: usize) -> String {
    let filled = if total > 0 {
        (completed as f64 / total as f64 * width as f64).round() as usize
    } else {
        0
    };
    let empty = width.saturating_sub(filled);
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}
