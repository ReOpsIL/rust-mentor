// src/ui/mod.rs
// Rendering. Views read the `App` state; they never change it (scroll limits
// measured while rendering are stored in `Cell`s).
mod application;
mod learning;
mod modals;
mod picker;
mod questions;
mod settings;
pub mod text;
mod welcome;

use crate::app::{App, AppState};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

/// Rendered lines keyed by (content version, width), so expensive layout
/// (syntax highlighting) is only redone when either changes
type CachedLines = Option<((u64, u16), Vec<Line<'static>>)>;

pub const SELECTED: Style = Style::new().fg(Color::Black).bg(Color::LightYellow);
const ACCENT: Style = Style::new().fg(Color::LightYellow).add_modifier(Modifier::BOLD);

pub fn render(frame: &mut Frame, app: &App) {
    let [title_area, main_area, footer_area] = Layout::vertical([
        Constraint::Length(2), // Title bar
        Constraint::Min(0),    // Main content
        Constraint::Length(2), // Key hints
    ])
    .areas(frame.area());

    render_title_bar(frame, app, title_area);

    let hints = match app.current_state {
        AppState::Welcome | AppState::LevelTooLowPopup => welcome::render_level_selection(frame, app, main_area),
        AppState::IndexSelection => welcome::render_index_selection(frame, app, main_area),
        AppState::Learning => learning::render(frame, app, main_area),
        AppState::Loading | AppState::QuestionGeneration | AppState::ApplicationGeneration => {
            modals::render_loading(frame, app, main_area)
        }
        AppState::Settings => settings::render(frame, app, main_area),
        AppState::QuestionAnswering => questions::render(frame, app, main_area),
        AppState::ApplicationDisplay => application::render(frame, app, main_area),
    };
    render_footer(frame, hints, footer_area);

    // Popups over everything else
    if app.current_state == AppState::LevelTooLowPopup {
        modals::render_level_too_low(frame);
    }
    picker::render_model_picker(frame, app);
    picker::render_topic_picker(frame, app);
    if app.show_help {
        modals::render_help(frame, app);
    }
    if app.show_quit_confirmation {
        modals::render_quit(frame, app);
    }
    if let Some((message, _)) = &app.status_message {
        modals::render_status_message(frame, message, main_area);
    }
    if let Some(error) = &app.last_error {
        modals::render_error(frame, error);
    }
}

fn render_title_bar(frame: &mut Frame, app: &App, area: Rect) {
    let context = match app.current_state {
        AppState::Welcome | AppState::LevelTooLowPopup => String::new(),
        AppState::Settings => " :: Settings".to_string(),
        _ => format!(" :: Level {} :: {}", app.selected_level, app.config().model),
    };
    let title = Paragraph::new(Line::from(vec![
        Span::styled(format!("Rust AI Mentor v{}", env!("CARGO_PKG_VERSION")), ACCENT),
        Span::styled(context, text::DIM),
    ]))
    .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(title, area);
}

/// Renders key hints given as `(key, action)` pairs
fn render_footer(frame: &mut Frame, hints: &[(&str, &str)], area: Rect) {
    let mut spans = Vec::new();
    for (i, (key, action)) in hints.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" │ ", text::DIM));
        }
        spans.push(Span::styled(*key, ACCENT));
        spans.push(Span::raw(format!(" {}", action)));
    }
    let footer =
        Paragraph::new(Line::from(spans)).alignment(Alignment::Center).block(Block::default().borders(Borders::TOP));
    frame.render_widget(footer, area);
}

/// A list line that is highlighted when selected
pub fn selectable_line(text: String, selected: bool) -> Line<'static> {
    if selected { Line::from(Span::styled(format!("> {}", text), SELECTED)) } else { Line::from(format!("  {}", text)) }
}

/// A rectangle of the given size (in cells, clamped to `area`) centered in `area`
pub fn centered(width: u16, height: u16, area: Rect) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);
    Rect { x: area.x + (area.width - width) / 2, y: area.y + (area.height - height) / 2, width, height }
}

/// Renders a widget over whatever is below it
pub fn render_modal(frame: &mut Frame, area: Rect, widget: impl Widget) {
    frame.render_widget(Clear, area);
    frame.render_widget(widget, area);
}

/// Renders the visible part of `lines` and records the scroll limits
pub fn render_scrolled(frame: &mut Frame, lines: &[Line<'static>], scroll: &crate::app::Scroll, area: Rect) {
    let height = area.height as usize;
    let max = lines.len().saturating_sub(height);
    scroll.max.set(max.min(u16::MAX as usize) as u16);
    scroll.page.set(area.height);
    let offset = scroll.clamped() as usize;
    let visible: Vec<Line> = lines.iter().skip(offset).take(height).cloned().collect();
    frame.render_widget(Paragraph::new(visible), area);

    // Scroll position indicator
    if max > 0 && area.width > 0 {
        let percent = offset * 100 / max;
        let label = format!(" {}% ", percent);
        let label_area = Rect {
            x: area.x + area.width.saturating_sub(label.len() as u16),
            y: area.y + area.height.saturating_sub(1),
            width: (label.len() as u16).min(area.width),
            height: 1,
        };
        frame.render_widget(Paragraph::new(Span::styled(label, SELECTED)), label_area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{AppState, ModelPicker, TopicPicker, TopicPurpose};
    use crate::config::{Config, ConfigService};
    use crate::llm::Model;
    use crate::model::{CodeSnippet, Exercise, LearningModule};
    use crate::question_generator::{AnswerOption, GeneratedApplication, Question, QuestionSet, QuestionType};
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    fn populated_app() -> App {
        let (mut app, _rx) = App::new(String::new(), ConfigService::in_memory(Config::default()));
        app.current_module = Some(LearningModule {
            topic: "Ownership: Moves".to_string(),
            explanation: "# Ownership\nSome **bold** text with `code`.\n\n- item one\n```rust\nlet x = 1;\n```"
                .to_string(),
            code_snippets: vec![CodeSnippet {
                title: "Example".to_string(),
                description: "An example".to_string(),
                code: "fn main() {\n    println!(\"hello, a fairly long line of output text\");\n}".to_string(),
            }],
            exercises: vec![Exercise {
                name: "Exercise".to_string(),
                description: "Do it".to_string(),
                code: "fn f() { todo!() }".to_string(),
            }],
            additional_resources: crate::resources::for_topic("Ownership Moves", &Default::default()),
        });
        app.question_set = Some(QuestionSet::new(
            "Ownership".to_string(),
            vec![Question {
                id: 0,
                text: "Which app would you like to build with a rather long question text?".to_string(),
                question_type: QuestionType::Multiple,
                options: (1..=4)
                    .map(|i| AnswerOption { id: i.to_string(), text: format!("Option number {}", i) })
                    .collect(),
                selected_answer: Some("2".to_string()),
            }],
        ));
        app.generated_application = Some(GeneratedApplication {
            name: "Quest".to_string(),
            description: "A game".to_string(),
            features: vec!["Fun".to_string()],
            code_snippets: vec![CodeSnippet {
                title: "Main Code".to_string(),
                description: String::new(),
                code: "fn main() {}".to_string(),
            }],
        });
        app.stream_preview = "<<<explanation: streaming>>>\nsome text".to_string();
        app.stream_chars = 30;
        app
    }

    #[test]
    fn every_screen_renders_at_any_size() {
        let states = [
            AppState::Welcome,
            AppState::IndexSelection,
            AppState::Learning,
            AppState::Loading,
            AppState::LevelTooLowPopup,
            AppState::Settings,
            AppState::QuestionGeneration,
            AppState::QuestionAnswering,
            AppState::ApplicationGeneration,
            AppState::ApplicationDisplay,
        ];
        for (width, height) in [(120, 40), (80, 24), (40, 12), (10, 5), (1, 1), (0, 0)] {
            let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
            for state in states {
                let mut app = populated_app();
                app.current_state = state;
                terminal.draw(|frame| render(frame, &app)).unwrap();

                // With every popup open as well
                app.show_help = true;
                app.show_quit_confirmation = true;
                app.last_error = Some("Something went wrong ".repeat(10));
                app.status_message = Some(("Saved".to_string(), std::time::Instant::now()));
                app.topic_picker = Some(TopicPicker {
                    topics: crate::data::topics_for_level(5, &crate::app::IndexType::Random).unwrap(),
                    filter: "own".to_string(),
                    cursor: 1,
                    purpose: TopicPurpose::StartQuestions,
                });
                app.model_picker = Some(ModelPicker {
                    models: Some(vec![Model { id: "a/b:free".to_string(), name: "B".to_string(), is_free: true }]),
                    filter: String::new(),
                    cursor: 0,
                });
                terminal.draw(|frame| render(frame, &app)).unwrap();
            }
        }
    }

    #[test]
    fn learning_view_measures_scroll_limits() {
        let mut terminal = Terminal::new(TestBackend::new(60, 15)).unwrap();
        let mut app = populated_app();
        app.current_state = AppState::Learning;
        terminal.draw(|frame| render(frame, &app)).unwrap();
        assert!(app.learning_scroll.max.get() > 0);
        assert!(app.learning_scroll.page.get() > 0);
    }
}
