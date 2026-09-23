// src/ui/application.rs
// The generated application view.
use super::text::{self, BOLD, DIM, HEADING};
use crate::app::App;
use crate::question_generator::GeneratedApplication;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use std::cell::RefCell;

thread_local! {
    static CACHE: RefCell<super::CachedLines> = const { RefCell::new(None) };
}

const HINTS: &[(&str, &str)] = &[
    ("↑↓/jk/PgUp/PgDn", "Scroll"),
    ("Enter/c", "Create project & continue"),
    ("Esc", "Continue to learning module"),
    ("?", "Help"),
];

pub fn render(frame: &mut Frame, app: &App, area: Rect) -> &'static [(&'static str, &'static str)] {
    let Some(application) = &app.generated_application else {
        let message = Paragraph::new("No application generated yet. Answer all questions to generate one.")
            .alignment(Alignment::Center);
        frame.render_widget(message, super::centered(area.width, 1, area));
        return HINTS;
    };

    let area = area.inner(Margin { horizontal: 1, vertical: 0 });
    let key = (app.application_version, area.width);
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if cache.as_ref().is_none_or(|(cached_key, _)| *cached_key != key) {
            *cache = Some((key, application_lines(application, area.width as usize)));
        }
        let (_, lines) = cache.as_ref().expect("cache filled above");
        super::render_scrolled(frame, lines, &app.application_scroll, area);
    });
    HINTS
}

fn application_lines(application: &GeneratedApplication, width: usize) -> Vec<Line<'static>> {
    let mut lines =
        text::wrap_spans(vec![Span::styled(format!("Application: {}", application.name), HEADING)], width, "");
    lines.push(Line::from(""));
    if !application.description.is_empty() {
        lines.extend(text::markdown(&application.description, width));
        lines.push(Line::from(""));
    }
    if !application.features.is_empty() {
        lines.push(Line::from(Span::styled("Features", BOLD)));
        for feature in &application.features {
            let mut spans = vec![Span::raw("• ")];
            spans.extend(text::inline_spans(feature, Style::default()));
            lines.extend(text::wrap_spans(spans, width, "  "));
        }
        lines.push(Line::from(""));
    }
    for snippet in &application.code_snippets {
        lines.extend(text::code_block(&snippet.code, width, Some(&snippet.title)));
        lines.push(Line::from(""));
    }
    lines
        .push(Line::from(Span::styled("The learning module for this topic is being prepared in the background.", DIM)));
    lines
}
