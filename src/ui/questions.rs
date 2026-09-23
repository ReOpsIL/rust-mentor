// src/ui/questions.rs
// Answering the questions that shape the generated application.
use super::SELECTED;
use super::text::{self, BOLD, DIM};
use crate::app::App;
use crate::question_generator::QuestionType;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Gauge, Paragraph};

const HINTS: &[(&str, &str)] =
    &[("←→/hl", "Question"), ("y/n 1-4 a-d", "Answer"), ("Enter", "Generate"), ("Esc", "Back"), ("?", "Help")];

pub fn render(frame: &mut Frame, app: &App, area: Rect) -> &'static [(&'static str, &'static str)] {
    let Some(question_set) = &app.question_set else {
        let message = Paragraph::new("No questions available. Press 'w' on the learning screen to generate questions.")
            .alignment(Alignment::Center);
        frame.render_widget(message, super::centered(area.width, 1, area));
        return HINTS;
    };

    let area = area.inner(Margin { horizontal: 2, vertical: 0 });
    let [topic_area, progress_area, question_area, options_area, note_area] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Min(4),
        Constraint::Min(4),
        Constraint::Length(1),
    ])
    .areas(area);

    let topic =
        Paragraph::new(Span::styled(format!("Topic: {}", question_set.topic), BOLD)).alignment(Alignment::Center);
    frame.render_widget(topic, topic_area);

    let (answered, total) = question_set.progress();
    let gauge = Gauge::default()
        .gauge_style(Style::new().fg(Color::LightYellow).bg(Color::DarkGray))
        .ratio(if total > 0 { answered as f64 / total as f64 } else { 0.0 })
        .label(format!("Question {}/{} — {} answered", question_set.current_question_index + 1, total, answered));
    frame.render_widget(gauge, Rect { height: 1, ..progress_area });

    let Some(question) = question_set.current_question() else {
        return HINTS;
    };

    let inner_width = question_area.width.saturating_sub(2) as usize;
    let question_widget =
        Paragraph::new(text::wrap_spans(text::inline_spans(&question.text, Style::default()), inner_width, ""))
            .block(Block::default().borders(Borders::ALL).title(" Question "));
    frame.render_widget(question_widget, question_area);

    let inner_width = options_area.width.saturating_sub(2) as usize;
    let mut option_lines = Vec::new();
    match question.question_type {
        QuestionType::Binary => {
            let style_for = |answer: &str| {
                if question.selected_answer.as_deref() == Some(answer) { SELECTED } else { Style::default() }
            };
            option_lines.push(Line::from(vec![
                Span::styled(" (Y) Yes ", style_for("Yes")),
                Span::raw("   "),
                Span::styled(" (N) No ", style_for("No")),
            ]));
        }
        QuestionType::Multiple => {
            for (i, option) in question.options.iter().enumerate() {
                let selected = question.selected_answer.as_deref() == Some(option.id.as_str());
                let letter = (b'a' + i as u8) as char;
                let label = if option.id.chars().all(|c| c.is_ascii_digit()) && i < 26 {
                    format!("({}/{}) ", option.id, letter)
                } else {
                    format!("({}) ", option.id)
                };
                let style = if selected { SELECTED } else { Style::default() };
                let indent = " ".repeat(label.len());
                let spans = vec![
                    Span::styled(label, style.add_modifier(Modifier::BOLD)),
                    Span::styled(option.text.clone(), style),
                ];
                option_lines.extend(text::wrap_spans(spans, inner_width, &indent));
            }
        }
    }
    let options = Paragraph::new(option_lines).block(Block::default().borders(Borders::ALL).title(" Answer "));
    frame.render_widget(options, options_area);

    let note = if question_set.is_complete() {
        Span::styled("All questions answered — press Enter to continue", Style::new().fg(Color::LightGreen))
    } else {
        Span::styled("Answering moves to the next unanswered question", DIM)
    };
    frame.render_widget(Paragraph::new(note).alignment(Alignment::Center), note_area);
    HINTS
}
