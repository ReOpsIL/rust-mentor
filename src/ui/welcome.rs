// src/ui/welcome.rs
// Level and content source selection.
use super::{selectable_line, text};
use crate::app::{App, INDEX_OPTIONS};
use crate::prompts::level_description;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub fn render_level_selection(frame: &mut Frame, app: &App, area: Rect) -> &'static [(&'static str, &'static str)] {
    let mut lines = vec![
        Line::from(Span::styled("Welcome to the Rust AI Mentor!", text::BOLD)),
        Line::from(Span::styled("A focused, terminal-based guide to your Rust journey.", text::DIM)),
        Line::from(""),
        Line::from("Select your current Rust skill level:"),
        Line::from(""),
    ];
    for level in 1..=10u8 {
        lines.push(selectable_line(
            format!("Level {:>2}: {:<22}", level, level_description(level)),
            level == app.selected_level,
        ));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("[ Press Enter to Begin ]", text::BOLD)));

    let height = (lines.len() as u16).min(area.height);
    let content = super::centered(area.width, height, area);
    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), content);

    &[("↑↓/jk/1-0", "Level"), ("Enter", "Continue"), ("s", "Settings"), ("?", "Help"), ("q", "Quit")]
}

pub fn render_index_selection(frame: &mut Frame, app: &App, area: Rect) -> &'static [(&'static str, &'static str)] {
    let mut lines = vec![Line::from(Span::styled("Select a content source:", text::BOLD)), Line::from("")];
    for (i, (_, label)) in INDEX_OPTIONS.iter().enumerate() {
        lines.push(selectable_line(format!("{:<56}", label), i == app.index_selection_cursor));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Next, choose a topic from the source (or a random or custom one). A few questions about the app",
        text::DIM,
    )));
    lines.push(Line::from(Span::styled(
        "you'd like to build follow, then the application and a learning module are generated.",
        text::DIM,
    )));

    let height = (lines.len() as u16).min(area.height);
    let content = super::centered(area.width, height, area);
    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), content);

    &[("↑↓/jk", "Source"), ("Enter", "Choose topic"), ("Esc", "Back"), ("s", "Settings"), ("?", "Help"), ("q", "Quit")]
}
