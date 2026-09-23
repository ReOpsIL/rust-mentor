// src/ui/learning.rs
// The learning module view: explanation, code examples, exercises and resources.
use super::text::{self, BOLD, DIM, HEADING, LINK};
use crate::app::App;
use crate::model::LearningModule;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use std::cell::RefCell;

thread_local! {
    // Rendered lines of the current module, keyed by (module version, width).
    // Highlighting is expensive, so it is only redone when either changes.
    static CACHE: RefCell<super::CachedLines> = const { RefCell::new(None) };
}

const HINTS: &[(&str, &str)] = &[
    ("↑↓/jk/PgUp/PgDn", "Scroll"),
    ("n", "Random module"),
    ("t", "Choose topic"),
    ("w", "Questions"),
    ("c", "Create project"),
    ("[ ]", "History"),
    ("s", "Settings"),
    ("?", "Help"),
    ("q", "Quit"),
];

pub fn render(frame: &mut Frame, app: &App, area: Rect) -> &'static [(&'static str, &'static str)] {
    let Some(module) = &app.current_module else {
        let placeholder =
            Paragraph::new("No learning module loaded. Press 'n' to generate one.").alignment(Alignment::Center);
        frame.render_widget(placeholder, super::centered(area.width, 1, area));
        return HINTS;
    };

    let area = area.inner(Margin { horizontal: 1, vertical: 0 });
    let [header_area, content_area] = Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).areas(area);

    let history = if app.history.len() > 1 {
        format!("  (module {} of {})", app.history_index + 1, app.history.len())
    } else {
        String::new()
    };
    let header = Paragraph::new(vec![
        Line::from(vec![Span::styled(format!("TOPIC: {}", module.topic), HEADING), Span::styled(history, DIM)]),
        Line::from(""),
    ]);
    frame.render_widget(header, header_area);

    let key = (app.module_version, content_area.width);
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if cache.as_ref().is_none_or(|(cached_key, _)| *cached_key != key) {
            *cache = Some((key, module_lines(module, content_area.width as usize)));
        }
        let (_, lines) = cache.as_ref().expect("cache filled above");
        super::render_scrolled(frame, lines, &app.learning_scroll, content_area);
    });
    HINTS
}

fn section_title(title: &str) -> Line<'static> {
    Line::from(Span::styled(title.to_string(), HEADING))
}

/// Lays out the whole module for the given width
pub fn module_lines(module: &LearningModule, width: usize) -> Vec<Line<'static>> {
    let mut lines = text::markdown(&module.explanation, width);

    if !module.code_snippets.is_empty() {
        lines.push(Line::from(""));
        lines.push(section_title("Code Examples"));
        for (i, snippet) in module.code_snippets.iter().enumerate() {
            lines.push(Line::from(""));
            lines.extend(text::wrap_spans(
                vec![Span::styled(format!("{}. {}", i + 1, snippet.title), BOLD)],
                width,
                "   ",
            ));
            if !snippet.description.is_empty() {
                lines.extend(text::wrap_text(&snippet.description, width, DIM));
            }
            lines.extend(text::code_block(&snippet.code, width, Some("Rust")));
        }
    }

    if !module.exercises.is_empty() {
        lines.push(Line::from(""));
        lines.push(section_title("Exercises"));
        for (i, exercise) in module.exercises.iter().enumerate() {
            lines.push(Line::from(""));
            lines.extend(text::wrap_spans(
                vec![Span::styled(format!("Exercise {}: {}", i + 1, exercise.name), BOLD)],
                width,
                "   ",
            ));
            if !exercise.description.is_empty() {
                lines.extend(text::wrap_spans(text::inline_spans(&exercise.description, Style::default()), width, ""));
            }
            if !exercise.code.is_empty() {
                lines.extend(text::code_block(&exercise.code, width, Some("Starter code")));
            }
        }
    }

    if let Some(resources) = &module.additional_resources {
        lines.push(Line::from(""));
        lines.push(section_title("Additional Learning Resources"));
        for group in &resources.groups {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(group.title, BOLD)));
            for resource in &group.resources {
                lines.extend(text::wrap_spans(
                    vec![Span::raw(format!("• {}: ", resource.title)), Span::styled(resource.url.clone(), LINK)],
                    width,
                    "  ",
                ));
                lines.extend(text::wrap_spans(
                    vec![Span::raw("  "), Span::styled(resource.description.clone(), DIM)],
                    width,
                    "  ",
                ));
            }
        }
    }
    lines
}
