// src/ui/modals.rs
// Loading screen, popups and the status line.
use super::text::{self, BOLD, DIM};
use super::{SELECTED, centered, render_modal};
use crate::app::{App, AppState};
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};

const SPINNER: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub fn render_loading(frame: &mut Frame, app: &App, area: Rect) -> &'static [(&'static str, &'static str)] {
    let message = match app.current_state {
        AppState::QuestionGeneration => "Generating questions",
        AppState::ApplicationGeneration => "Generating your application",
        _ => "Generating your learning module",
    };
    let spinner = SPINNER[app.tick_count() as usize % SPINNER.len()];
    let status = if app.stream_chars == 0 {
        "Waiting for the model...".to_string()
    } else {
        format!("{} characters received", app.stream_chars)
    };

    let [_, message_area, preview_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Length(3), Constraint::Min(0)]).areas(area);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(Span::styled(format!("{} {}...", spinner, message), BOLD)),
            Line::from(Span::styled(status, DIM)),
        ])
        .alignment(Alignment::Center),
        message_area,
    );

    // Live preview of the streamed text (the last lines that fit)
    if !app.stream_preview.is_empty() && preview_area.height > 2 {
        let preview_area = preview_area.inner(Margin { horizontal: 2, vertical: 0 });
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(DIM)
            .title(" Live preview ");
        let inner = block.inner(preview_area);
        let lines = text::wrap_text(&app.stream_preview, inner.width as usize, DIM);
        let visible: Vec<Line> =
            lines.iter().skip(lines.len().saturating_sub(inner.height as usize)).cloned().collect();
        frame.render_widget(Paragraph::new(visible).block(block), preview_area);
    }

    &[("Esc", "Cancel"), ("?", "Help"), ("q", "Quit")]
}

pub fn render_help(frame: &mut Frame, app: &App) {
    let sections: &[(&str, &[(&str, &str)])] = &[
        (
            "Global",
            &[("?", "Toggle help"), ("q", "Quit"), ("s", "Open settings"), ("Esc", "Back / cancel while generating")],
        ),
        (
            "Level & source selection",
            &[("↑↓ / j k", "Change selection"), ("1-9, 0", "Pick level 1-10"), ("Enter", "Continue")],
        ),
        (
            "Topic picker",
            &[
                ("type", "Filter topics (the text can also be a custom topic)"),
                ("↑↓ PgUp PgDn", "Move"),
                ("Enter", "Choose (\"Random topic\" picks one for you)"),
                ("Esc", "Cancel"),
            ],
        ),
        (
            "Questions",
            &[
                ("← → / h l", "Previous / next question"),
                ("y / n", "Answer yes/no questions"),
                ("1-4 / a-d", "Answer multiple choice questions"),
                ("Enter", "Generate the application (all answered)"),
            ],
        ),
        (
            "Application",
            &[
                ("↑↓ PgUp PgDn", "Scroll"),
                ("Enter / c", "Create Cargo project and continue"),
                ("Esc", "Continue to the learning module"),
            ],
        ),
        (
            "Learning module",
            &[
                ("↑↓ / j k", "Scroll"),
                ("PgUp PgDn / b Space", "Scroll a page"),
                ("g / G", "Top / bottom"),
                ("n", "New module on a random topic"),
                ("t", "New module on a topic you choose"),
                ("w", "Questions for this topic"),
                ("c", "Create a Cargo project from this module"),
                ("[ / ]", "Previous / next module in history"),
            ],
        ),
        (
            "Settings",
            &[
                ("Tab / Shift+Tab", "Switch section"),
                ("↑↓ / j k", "Navigate options"),
                ("← → / h l / Enter", "Change the selected option"),
                ("Esc", "Return to the previous screen"),
            ],
        ),
    ];

    let mut lines = Vec::new();
    for (title, keys) in sections {
        lines.push(Line::from(Span::styled(*title, text::HEADING)));
        for (key, action) in *keys {
            lines.push(Line::from(vec![Span::styled(format!("  {:<22}", key), BOLD), Span::raw(*action)]));
        }
        lines.push(Line::from(""));
    }

    let area = centered(72, lines.len() as u16 + 2, frame.area());
    let block = Block::default()
        .title(" Keybindings ")
        .title_bottom(Line::from(" Esc to close ").centered())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let inner = block.inner(area);
    render_modal(frame, area, block);
    super::render_scrolled(frame, &lines, &app.help_scroll, inner.inner(Margin { horizontal: 1, vertical: 0 }));
}

pub fn render_quit(frame: &mut Frame, app: &App) {
    let area = centered(40, 5, frame.area());
    let button = |label: &'static str, selected: bool| {
        Span::styled(format!(" {} ", label), if selected { SELECTED } else { Style::default() })
    };
    let content = Paragraph::new(vec![
        Line::from("Are you sure you want to quit?"),
        Line::from(""),
        Line::from(vec![
            button("Yes", app.quit_confirmation_selected),
            Span::raw("   "),
            button("No", !app.quit_confirmation_selected),
        ]),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    render_modal(frame, area, content);
}

pub fn render_level_too_low(frame: &mut Frame) {
    let area = centered(60, 5, frame.area());
    let content = Paragraph::new(vec![
        Line::from("Library topics need level 3 (or higher)."),
        Line::from(""),
        Line::from(Span::styled("Returning to level selection...", DIM)),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().title(" Level Too Low ").borders(Borders::ALL).border_type(BorderType::Rounded));
    render_modal(frame, area, content);
}

/// A one-line message at the bottom of the main content area
pub fn render_status_message(frame: &mut Frame, message: &str, area: Rect) {
    if area.height == 0 {
        return;
    }
    let line_area = Rect { y: area.y + area.height - 1, height: 1, ..area };
    let status =
        Paragraph::new(message).alignment(Alignment::Center).style(Style::new().fg(Color::Black).bg(Color::LightGreen));
    render_modal(frame, line_area, status);
}

pub fn render_error(frame: &mut Frame, error: &str) {
    let width = frame.area().width.saturating_sub(8).min(80);
    let text_lines = text::wrap_text(error, width.saturating_sub(4) as usize, Style::default());
    let height = (text_lines.len() as u16 + 4).min(frame.area().height);
    let area = centered(width, height, frame.area());

    let mut lines = text_lines;
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("Press any key to continue", BOLD)));
    let content = Paragraph::new(lines).wrap(Wrap { trim: false }).block(
        Block::default()
            .title(" Error ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(Color::LightRed)),
    );
    render_modal(frame, area, content);
}
