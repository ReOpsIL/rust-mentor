// src/ui/picker.rs
// Filterable list popups: the model picker and the topic picker.
use super::text::{BOLD, DIM};
use super::{SELECTED, centered, render_modal};
use crate::app::{App, TopicChoice, TopicPurpose};
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

const GREEN: Style = Style::new().fg(Color::LightGreen);

/// Renders a popup with a filter line and a list of rows; the row at `cursor` is highlighted.
/// `message` replaces the list (e.g. while loading).
fn render_list(frame: &mut Frame, title: &str, filter: &str, rows: Vec<Line<'static>>, cursor: usize, message: &str) {
    let area =
        centered(frame.area().width.saturating_sub(8).min(100), frame.area().height.saturating_sub(4), frame.area());
    let block = Block::default()
        .title(format!(" {} ", title))
        .title_bottom(Line::from(" type to filter · ↑↓ PgUp PgDn move · Enter choose · Esc cancel ").centered())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let inner = block.inner(area);
    render_modal(frame, area, block);

    let [filter_area, list_area] = Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).areas(inner);
    frame.render_widget(
        Paragraph::new(Line::from(vec![Span::styled("Filter: ", BOLD), Span::raw(format!("{}▏", filter))])),
        filter_area,
    );

    if rows.is_empty() {
        frame.render_widget(Paragraph::new(Span::styled(message.to_string(), DIM)), list_area);
        return;
    }

    // Keep the cursor visible
    let height = list_area.height as usize;
    let first = cursor.saturating_sub(height.saturating_sub(1));
    let lines: Vec<Line> = rows
        .into_iter()
        .enumerate()
        .skip(first)
        .take(height)
        .map(|(i, row)| {
            if i == cursor {
                let text: String = row.spans.iter().map(|s| s.content.as_ref()).collect();
                Line::from(Span::styled(text, SELECTED))
            } else {
                row
            }
        })
        .collect();
    frame.render_widget(Paragraph::new(lines), list_area);
}

pub fn render_model_picker(frame: &mut Frame, app: &App) {
    let Some(picker) = &app.model_picker else {
        return;
    };
    let current = &app.config().model;
    let rows = picker
        .filtered()
        .into_iter()
        .map(|model| {
            let marker = if &model.id == current { "● " } else { "  " };
            Line::from(vec![
                Span::raw(format!("{}{}", marker, model.id)),
                Span::styled(if model.is_free { " [free]" } else { "" }, GREEN),
                Span::styled(format!("  {}", model.name), DIM),
            ])
        })
        .collect();
    let message = if picker.models.is_none() { "Loading models from OpenRouter..." } else { "No matching models" };
    render_list(frame, "Select a model", &picker.filter, rows, picker.cursor, message);
}

pub fn render_topic_picker(frame: &mut Frame, app: &App) {
    let Some(picker) = &app.topic_picker else {
        return;
    };
    let rows = picker
        .choices()
        .into_iter()
        .map(|choice| match choice {
            TopicChoice::Random => Line::from(vec![
                Span::styled("  Random topic", BOLD),
                Span::styled("  let RustMentor pick one for your level", DIM),
            ]),
            TopicChoice::Custom(text) => {
                Line::from(vec![Span::styled("  Custom topic: ", GREEN), Span::raw(format!("\"{}\"", text))])
            }
            TopicChoice::Listed(topic) => Line::from(vec![
                Span::raw(format!("  {}", topic.topic)),
                Span::styled(format!("  {}", topic.source), DIM),
            ]),
        })
        .collect();
    let title = match picker.purpose {
        TopicPurpose::StartQuestions => format!("Choose a topic (level {})", app.selected_level),
        TopicPurpose::NextModule => format!("Choose the topic of the next module (level {})", app.selected_level),
    };
    render_list(frame, &title, &picker.filter, rows, picker.cursor, "");
}
