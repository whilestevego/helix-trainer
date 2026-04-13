//! Tests for `handle_event` — the pure TUI state-machine dispatcher.

mod common;

use std::path::PathBuf;
use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use helixir::tui::action::{Action, FLASH_DURATION, handle_event};
use helixir::tui::app::{ExerciseStatus, Panel, TreeCursor};
use helixir::tui::event::AppEvent;

use common::test_app;

fn key(code: KeyCode) -> AppEvent {
    AppEvent::Key(KeyEvent::new(code, KeyModifiers::NONE))
}

fn key_with(code: KeyCode, mods: KeyModifiers) -> AppEvent {
    AppEvent::Key(KeyEvent::new(code, mods))
}

fn dispatch(app: &mut helixir::tui::app::App, ev: AppEvent) -> Action {
    handle_event(app, ev, Instant::now())
}

#[test]
fn q_key_quits() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    assert_eq!(dispatch(&mut app, key(KeyCode::Char('q'))), Action::Quit);
    assert!(app.quit);
}

#[test]
fn ctrl_c_quits() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    let ev = key_with(KeyCode::Char('c'), KeyModifiers::CONTROL);
    assert_eq!(dispatch(&mut app, ev), Action::Quit);
    assert!(app.quit);
}

#[test]
fn j_in_list_advances_cursor() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    // Initial cursor: Exercise 1 (Movement/m2, first non-passed).
    assert_eq!(app.cursor, TreeCursor::Exercise(1));
    dispatch(&mut app, key(KeyCode::Char('j')));
    // Next visible node after Exercise(1) is the Selection module header
    // (Selection is collapsed by default).
    assert_eq!(app.cursor, TreeCursor::Module("Selection".to_string()));
}

#[test]
fn k_at_top_does_not_move() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    // Move to the Movement module header (top of the visible tree).
    app.cursor = TreeCursor::Module("Movement".to_string());
    dispatch(&mut app, key(KeyCode::Char('k')));
    assert_eq!(app.cursor, TreeCursor::Module("Movement".to_string()));
}

#[test]
fn j_in_detail_focus_scrolls() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    app.focused_panel = Panel::Detail;
    app.detail_scroll_max = 100;
    dispatch(&mut app, key(KeyCode::Char('j')));
    assert_eq!(app.detail_scroll, 3);
}

#[test]
fn k_in_detail_focus_scrolls_up() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    app.focused_panel = Panel::Detail;
    app.detail_scroll_max = 100;
    app.detail_scroll = 10;
    dispatch(&mut app, key(KeyCode::Char('k')));
    assert_eq!(app.detail_scroll, 7);
}

#[test]
fn h_and_l_switch_focus() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    dispatch(&mut app, key(KeyCode::Char('l')));
    assert_eq!(app.focused_panel, Panel::Detail);
    dispatch(&mut app, key(KeyCode::Char('h')));
    assert_eq!(app.focused_panel, Panel::List);
}

#[test]
fn n_jumps_to_next_incomplete() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    // Initial non-passed is index 1 (Movement/m2). Next non-passed is index 2
    // (Selection/s1).
    dispatch(&mut app, key(KeyCode::Char('n')));
    assert_eq!(app.cursor, TreeCursor::Exercise(2));
}

#[test]
fn space_reveals_hints_up_to_max() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    // Cursor starts on Exercise 1 (Movement/m2) which has 1 hint.
    dispatch(&mut app, key(KeyCode::Char(' ')));
    assert_eq!(app.hint_level, 1);
    // Further presses stay at max.
    dispatch(&mut app, key(KeyCode::Char(' ')));
    assert_eq!(app.hint_level, 1);
}

#[test]
fn space_on_module_header_does_nothing() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    app.cursor = TreeCursor::Module("Movement".to_string());
    app.hint_level = 0;
    dispatch(&mut app, key(KeyCode::Char(' ')));
    assert_eq!(app.hint_level, 0);
}

#[test]
fn question_toggles_help() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    dispatch(&mut app, key(KeyCode::Char('?')));
    assert!(app.show_help);
    dispatch(&mut app, key(KeyCode::Char('?')));
    assert!(!app.show_help);
}

#[test]
fn help_overlay_swallows_navigation() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    app.show_help = true;
    let before = app.cursor.clone();
    dispatch(&mut app, key(KeyCode::Char('j')));
    assert_eq!(app.cursor, before);
    // Esc closes help.
    dispatch(&mut app, key(KeyCode::Esc));
    assert!(!app.show_help);
}

#[test]
fn c_opens_cheatsheet_then_esc_closes_and_resets_scroll() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    dispatch(&mut app, key(KeyCode::Char('c')));
    assert!(app.show_cheatsheet);
    assert_eq!(app.cheatsheet_scroll, 0);
    // j scrolls down.
    dispatch(&mut app, key(KeyCode::Char('j')));
    assert_eq!(app.cheatsheet_scroll, 3);
    dispatch(&mut app, key(KeyCode::Char('k')));
    assert_eq!(app.cheatsheet_scroll, 0);
    dispatch(&mut app, key(KeyCode::Esc));
    assert!(!app.show_cheatsheet);
    assert_eq!(app.cheatsheet_scroll, 0);
}

#[test]
fn z_chord_zc_collapses_current_module() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    // Movement is expanded initially. zc collapses it.
    dispatch(&mut app, key(KeyCode::Char('z')));
    assert_eq!(app.pending_chord, Some('z'));
    dispatch(&mut app, key(KeyCode::Char('c')));
    assert!(app.pending_chord.is_none());
    assert!(app.is_module_collapsed("Movement"));
    // Cursor was on a Movement exercise and should have been promoted to
    // the module header (fix_stranded_cursor).
    assert_eq!(app.cursor, TreeCursor::Module("Movement".to_string()));
}

#[test]
fn z_chord_zo_expands_current_module() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    app.collapse_current_module();
    dispatch(&mut app, key(KeyCode::Char('z')));
    dispatch(&mut app, key(KeyCode::Char('o')));
    assert!(!app.is_module_collapsed("Movement"));
}

#[test]
fn z_chord_za_toggles_current_module() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    let before = app.is_module_collapsed("Movement");
    dispatch(&mut app, key(KeyCode::Char('z')));
    dispatch(&mut app, key(KeyCode::Char('a')));
    assert_eq!(app.is_module_collapsed("Movement"), !before);
}

#[test]
fn z_chord_capital_m_collapses_all_modules() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    dispatch(&mut app, key(KeyCode::Char('z')));
    dispatch(&mut app, key(KeyCode::Char('M')));
    assert!(app.is_module_collapsed("Movement"));
    assert!(app.is_module_collapsed("Selection"));
}

#[test]
fn z_chord_capital_r_expands_all_modules() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    app.collapse_all_modules();
    dispatch(&mut app, key(KeyCode::Char('z')));
    dispatch(&mut app, key(KeyCode::Char('R')));
    assert!(!app.is_module_collapsed("Movement"));
    assert!(!app.is_module_collapsed("Selection"));
}

#[test]
fn z_chord_unrecognized_clears_pending() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    let before_collapsed = app.collapsed_modules.clone();
    dispatch(&mut app, key(KeyCode::Char('z')));
    dispatch(&mut app, key(KeyCode::Char('x'))); // unrecognized
    assert!(app.pending_chord.is_none());
    assert_eq!(app.collapsed_modules, before_collapsed);
}

#[test]
fn tab_toggles_current_module() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    let before = app.is_module_collapsed("Movement");
    dispatch(&mut app, key(KeyCode::Tab));
    assert_eq!(app.is_module_collapsed("Movement"), !before);
}

#[test]
fn r_returns_reset_action() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    assert_eq!(
        dispatch(&mut app, key(KeyCode::Char('r'))),
        Action::ResetCurrent
    );
}

#[test]
fn u_returns_install_missing_action() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    assert_eq!(
        dispatch(&mut app, key(KeyCode::Char('u'))),
        Action::InstallMissing
    );
}

#[test]
fn tick_without_flash_is_noop() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    assert!(app.flash_message.is_none());
    let action = dispatch(&mut app, AppEvent::Tick);
    assert_eq!(action, Action::None);
    assert!(app.flash_message.is_none());
}

#[test]
fn tick_with_fresh_flash_preserves_it() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    app.flash_message = Some(("🎉 PASSED! Auto-advancing...".to_string(), Instant::now()));
    handle_event(&mut app, AppEvent::Tick, Instant::now());
    assert!(app.flash_message.is_some());
}

#[test]
fn tick_with_expired_passed_flash_advances() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    // Park cursor on Movement/m2 (index 1, Failed).
    app.cursor = TreeCursor::Exercise(1);
    let created = Instant::now() - FLASH_DURATION - Duration::from_millis(100);
    app.flash_message = Some(("🎉 PASSED! Auto-advancing...".to_string(), created));
    handle_event(&mut app, AppEvent::Tick, Instant::now());
    assert!(app.flash_message.is_none());
    // Next incomplete after index 1 is index 2 (Selection/s1).
    assert_eq!(app.cursor, TreeCursor::Exercise(2));
}

#[test]
fn tick_with_expired_non_passed_flash_only_clears() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    let before_cursor = app.cursor.clone();
    let created = Instant::now() - FLASH_DURATION - Duration::from_millis(100);
    app.flash_message = Some(("📦 Installed 3 new exercises!".to_string(), created));
    handle_event(&mut app, AppEvent::Tick, Instant::now());
    assert!(app.flash_message.is_none());
    assert_eq!(app.cursor, before_cursor);
}

#[test]
fn file_changed_for_focused_passing_exercise_flashes() {
    // Write a passing .hxt to disk so reverify_by_path actually succeeds.
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().to_path_buf();
    let mut app = test_app(dir.clone());
    // Focus exercise 1 (Movement/m2) and prepare its file with passing content.
    app.cursor = TreeCursor::Exercise(1);
    let path = app.exercises[1].file_path.clone();
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(
        &path,
        "\
────────────────────────── PRACTICE ──────────────────────────────

hello

────────────────────────── EXPECTED ──────────────────────────────

hello

──────────────────────────────────────────────────────────────────
",
    )
    .unwrap();

    handle_event(&mut app, AppEvent::FileChanged(path), Instant::now());
    assert_eq!(app.exercises[1].status, ExerciseStatus::Passed);
    assert!(app.flash_message.is_some());
}

#[test]
fn file_changed_for_unknown_path_is_noop() {
    let mut app = test_app(PathBuf::from("/tmp/x"));
    let before_status: Vec<_> = app.exercises.iter().map(|e| e.status.clone()).collect();
    handle_event(
        &mut app,
        AppEvent::FileChanged(PathBuf::from("/nowhere/else.hxt")),
        Instant::now(),
    );
    let after_status: Vec<_> = app.exercises.iter().map(|e| e.status.clone()).collect();
    assert_eq!(before_status, after_status);
    assert!(app.flash_message.is_none());
}
