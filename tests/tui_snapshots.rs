//! Snapshot tests for `tui::ui::render` at representative `App` states.
//!
//! Uses `ratatui::backend::TestBackend` to draw into an in-memory buffer,
//! then serializes the buffer row-by-row for `insta::assert_snapshot!`.

mod common;

use std::path::PathBuf;
use std::time::Instant;

use helixir::tui::app::{App, ExerciseStatus, Panel, TreeCursor};
use helixir::tui::ui;
use ratatui::Terminal;
use ratatui::backend::TestBackend;

use common::test_app;

const WIDTH: u16 = 120;
const HEIGHT: u16 = 40;

fn render_to_string(app: &mut App) -> String {
    let backend = TestBackend::new(WIDTH, HEIGHT);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| ui::render(frame, app)).unwrap();
    let buffer = terminal.backend().buffer().clone();
    let mut out = String::new();
    for y in 0..buffer.area.height {
        let mut line = String::new();
        for x in 0..buffer.area.width {
            line.push_str(buffer[(x, y)].symbol());
        }
        out.push_str(line.trim_end());
        out.push('\n');
    }
    out
}

fn app() -> App {
    test_app(PathBuf::from("/tmp/helixir-test"))
}

#[test]
fn snapshot_initial_state() {
    let mut a = app();
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_help_overlay() {
    let mut a = app();
    a.show_help = true;
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_cheatsheet_populated() {
    let mut a = app();
    a.show_cheatsheet = true;
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_cheatsheet_empty() {
    let mut a = app();
    // Mark everything NotStarted so the cheatsheet has no passed commands.
    for ex in &mut a.exercises {
        ex.status = ExerciseStatus::NotStarted;
    }
    a.show_cheatsheet = true;
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_failing_exercise() {
    let mut a = app();
    // Cursor starts on Exercise 1 (Failed).
    a.cursor = TreeCursor::Exercise(1);
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_passed_exercise() {
    let mut a = app();
    // Expand Selection so Exercise(3) (Passed) is visible, then cursor there.
    a.expand_all_modules();
    a.cursor = TreeCursor::Exercise(3);
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_not_started_exercise() {
    let mut a = app();
    a.expand_all_modules();
    a.cursor = TreeCursor::Exercise(2);
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_all_modules_collapsed() {
    let mut a = app();
    a.collapse_all_modules();
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_flash_message_visible() {
    let mut a = app();
    a.flash_message = Some(("🎉 PASSED! Auto-advancing...".to_string(), Instant::now()));
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_detail_panel_focused() {
    let mut a = app();
    a.focused_panel = Panel::Detail;
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_module_header_selected() {
    let mut a = app();
    a.cursor = TreeCursor::Module("Movement".to_string());
    insta::assert_snapshot!(render_to_string(&mut a));
}
