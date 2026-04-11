use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

use crate::tui::app::{App, ExerciseStatus};

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Header
            Constraint::Min(10),   // Main content
            Constraint::Length(1), // Footer
        ])
        .split(frame.area());

    render_header(frame, chunks[0], app);
    render_main(frame, chunks[1], app);
    render_footer(frame, chunks[2], app);

    if app.show_help {
        render_help_popup(frame, app);
    }
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let (mod_idx, mod_total) = app.current_module_index();
    let (ex_idx, ex_total) = app.current_exercise_in_module();

    let header = Line::from(vec![
        Span::styled(" 🏋️ Helix Trainer", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled(
            format!("📦 Module {}/{}  ", mod_idx, mod_total),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            format!("📝 Exercise {}/{}  ", ex_idx, ex_total),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled("[?]=help", Style::default().fg(Color::DarkGray)),
    ]);

    frame.render_widget(Paragraph::new(header), area);
}

fn render_main(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(24), Constraint::Min(40)])
        .split(area);

    render_exercise_list(frame, chunks[0], app);
    render_exercise_detail(frame, chunks[1], app);
}

fn render_exercise_list(frame: &mut Frame, area: Rect, app: &App) {
    let mut items: Vec<ListItem> = Vec::new();
    let mut current_category = String::new();

    for (i, exercise) in app.exercises.iter().enumerate() {
        if exercise.meta.category != current_category {
            current_category = exercise.meta.category.clone();
            items.push(ListItem::new(Line::from(Span::styled(
                format!("🗂 {}", current_category),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ))));
        }

        let (icon, style) = if i == app.selected {
            ("👉", Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        } else {
            match exercise.status {
                ExerciseStatus::Passed => ("✅", Style::default().fg(Color::Green)),
                ExerciseStatus::Failed => ("⬜", Style::default().fg(Color::White)),
                ExerciseStatus::NotStarted => ("⬜", Style::default().fg(Color::DarkGray)),
            }
        };

        let title = &exercise.meta.title;
        let num = exercise
            .meta
            .id
            .split('/')
            .last()
            .unwrap_or(&exercise.meta.id)
            .split('-')
            .next()
            .unwrap_or("");

        items.push(ListItem::new(Line::from(Span::styled(
            format!(" {} {}.{}", icon, num, title),
            style,
        ))));
    }

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::RIGHT)
            .title(" 📚 Exercises "),
    );

    frame.render_widget(list, area);
}

fn render_exercise_detail(frame: &mut Frame, area: Rect, app: &App) {
    let exercise = app.selected_exercise();
    let meta = exercise.meta;

    let difficulty_stars = match meta.difficulty {
        1 => "⭐",
        2 => "⭐⭐",
        _ => "⭐⭐⭐",
    };

    let mut lines: Vec<Line> = Vec::new();

    // Title
    lines.push(Line::from(Span::styled(
        format!(" {} {}", difficulty_stars, meta.title),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::raw(""));

    // Commands
    if !meta.commands.is_empty() {
        lines.push(Line::from(Span::styled(
            " ⌨️  COMMANDS",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        for cmd in &meta.commands {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("   {:8}", cmd.key),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
                Span::styled(&cmd.description, Style::default().fg(Color::White)),
            ]));
        }
        lines.push(Line::raw(""));
    }

    // Notes
    if !meta.notes.is_empty() {
        for line in meta.notes.lines() {
            lines.push(Line::from(Span::styled(
                format!("   {}", line),
                Style::default().fg(Color::DarkGray),
            )));
        }
        lines.push(Line::raw(""));
    }

    // Instructions
    lines.push(Line::from(Span::styled(
        " 📋 INSTRUCTIONS",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )));
    for line in meta.instructions.trim().lines() {
        lines.push(Line::from(Span::styled(
            format!("   {}", line),
            Style::default().fg(Color::White),
        )));
    }
    lines.push(Line::raw(""));

    // Status / Diff
    lines.push(Line::from(Span::styled(
        " ─────────────────────────────────────────",
        Style::default().fg(Color::DarkGray),
    )));

    // Flash message
    if let Some((msg, _)) = &app.flash_message {
        lines.push(Line::from(Span::styled(
            format!(" {}", msg),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
    } else {
        match &exercise.status {
            ExerciseStatus::Passed => {
                lines.push(Line::from(Span::styled(
                    " ✅ PASSED!",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )));
            }
            ExerciseStatus::NotStarted => {
                lines.push(Line::from(Span::styled(
                    " ⬜ Not started — edit the .hxt file to begin",
                    Style::default().fg(Color::DarkGray),
                )));
            }
            ExerciseStatus::Failed => {
                let diff_count = exercise.diff.len();
                lines.push(Line::from(Span::styled(
                    format!(" ❌ {} difference{} found", diff_count, if diff_count == 1 { "" } else { "s" }),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )));
                for d in exercise.diff.iter().take(5) {
                    lines.push(Line::from(Span::styled(
                        format!("   line {}: got \"{}\"", d.line_num, d.got),
                        Style::default().fg(Color::Red),
                    )));
                    lines.push(Line::from(Span::styled(
                        format!("          expected \"{}\"", d.expected),
                        Style::default().fg(Color::Green),
                    )));
                }
                if diff_count > 5 {
                    lines.push(Line::from(Span::styled(
                        format!("   ... and {} more", diff_count - 5),
                        Style::default().fg(Color::DarkGray),
                    )));
                }
            }
        }
    }

    // Hints
    if app.hint_level > 0 {
        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            " 💡 HINTS",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        for (i, hint) in meta.hints.iter().enumerate() {
            if i < app.hint_level {
                lines.push(Line::from(Span::styled(
                    format!("   {}. {}", i + 1, hint),
                    Style::default().fg(Color::Yellow),
                )));
            }
        }
        let remaining = meta.hints.len().saturating_sub(app.hint_level);
        if remaining > 0 {
            lines.push(Line::from(Span::styled(
                format!("   ({} more — press h)", remaining),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    lines.push(Line::raw(""));
    lines.push(Line::from(Span::styled(
        " ─────────────────────────────────────────",
        Style::default().fg(Color::DarkGray),
    )));

    // Keybinding bar
    lines.push(Line::from(vec![
        Span::styled(" 💡", Style::default()),
        Span::styled("[h]", Style::default().fg(Color::Cyan)),
        Span::styled(" hint  ", Style::default().fg(Color::DarkGray)),
        Span::styled("🔄", Style::default()),
        Span::styled("[r]", Style::default().fg(Color::Cyan)),
        Span::styled(" reset  ", Style::default().fg(Color::DarkGray)),
        Span::styled("⏭️", Style::default()),
        Span::styled("[n]", Style::default().fg(Color::Cyan)),
        Span::styled(" next  ", Style::default().fg(Color::DarkGray)),
        Span::styled("🚪", Style::default()),
        Span::styled("[q]", Style::default().fg(Color::Cyan)),
        Span::styled(" quit", Style::default().fg(Color::DarkGray)),
    ]));

    let detail = Paragraph::new(lines)
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap { trim: false });

    frame.render_widget(detail, area);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let completed = app.completed_count();
    let total = app.total_count();
    let pct = if total > 0 {
        (completed as f64 / total as f64 * 100.0).round() as usize
    } else {
        0
    };

    let bar_width = 20usize;
    let filled = if total > 0 {
        (completed as f64 / total as f64 * bar_width as f64).round() as usize
    } else {
        0
    };
    let bar = format!(
        "{}{}",
        "█".repeat(filled),
        "░".repeat(bar_width.saturating_sub(filled))
    );

    let footer = Line::from(vec![
        Span::styled(" 🏆 ", Style::default()),
        Span::styled(bar, Style::default().fg(Color::Green)),
        Span::styled(
            format!(" {}/{} ({}%)", completed, total, pct),
            Style::default().fg(Color::White),
        ),
        Span::styled(
            "                              👀 Watching...",
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    frame.render_widget(Paragraph::new(footer), area);
}

fn render_help_popup(frame: &mut Frame, _app: &App) {
    let area = centered_rect(50, 60, frame.area());

    let help_text = vec![
        Line::from(Span::styled(
            " 🏋️ Helix Trainer — Help",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::raw(""),
        Line::from(vec![
            Span::styled("  j/↓    ", Style::default().fg(Color::Green)),
            Span::raw("Next exercise"),
        ]),
        Line::from(vec![
            Span::styled("  k/↑    ", Style::default().fg(Color::Green)),
            Span::raw("Previous exercise"),
        ]),
        Line::from(vec![
            Span::styled("  h      ", Style::default().fg(Color::Green)),
            Span::raw("Reveal next hint"),
        ]),
        Line::from(vec![
            Span::styled("  n      ", Style::default().fg(Color::Green)),
            Span::raw("Jump to next incomplete"),
        ]),
        Line::from(vec![
            Span::styled("  r      ", Style::default().fg(Color::Green)),
            Span::raw("Reset current exercise"),
        ]),
        Line::from(vec![
            Span::styled("  ?      ", Style::default().fg(Color::Green)),
            Span::raw("Toggle this help"),
        ]),
        Line::from(vec![
            Span::styled("  q      ", Style::default().fg(Color::Green)),
            Span::raw("Quit"),
        ]),
        Line::raw(""),
        Line::from(Span::styled(
            "  Edit .hxt files in your editor — changes",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "  are detected automatically on save.",
            Style::default().fg(Color::DarkGray),
        )),
        Line::raw(""),
        Line::from(Span::styled(
            "  Press ? or Esc to close",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let popup = Paragraph::new(help_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" Help "),
    );

    // Clear the area first
    frame.render_widget(ratatui::widgets::Clear, area);
    frame.render_widget(popup, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
