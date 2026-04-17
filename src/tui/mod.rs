pub mod action;
pub mod app;
pub mod event;
pub mod ui;

use std::io;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use action::{Action, handle_event};
use app::App;
use event::EventHandler;

pub async fn run(exercises_dir: PathBuf) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(exercises_dir.clone())?;
    let mut events = EventHandler::new(exercises_dir, crate::metadata::exercise_extensions());

    // Main loop
    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        let event = events.next().await?;
        match handle_event(&mut app, event, Instant::now()) {
            Action::None => {}
            Action::Quit => {}
            Action::ResetCurrent => app.reset_current()?,
            Action::InstallMissing => app.install_missing_exercises()?,
        }

        if app.quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
