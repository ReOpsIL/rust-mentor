// src/ui.rs
use crate::app::{App, AppState, SettingsSection, LearningGoal};
use crate::config::{CodeComplexity, ExplanationVerbosity, FocusArea};
use lazy_static::lazy_static;
use ratatui::prelude::*;
use ratatui::widgets::*;
use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet};
use syntect::parsing::SyntaxSet;
use textwrap;
use textwrap::wrap;

// Initialize syntect resources once
lazy_static! {
    static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
}

pub fn render(frame: &mut Frame, app: &App) {
    // Main layout
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title bar
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Status bar
        ])
        .split(frame.size());

    // Render main content based on state
    match app.current_state {
        AppState::Welcome => render_welcome_view(frame, app, &main_layout),
        AppState::IndexSelection => render_index_selection_view(frame, app, &main_layout),
        AppState::Learning => render_learning_view(frame, app, &main_layout),
        AppState::Loading => render_loading_view(frame, app, &main_layout),
        AppState::Settings => render_settings_view(frame, app, &main_layout),
        AppState::LevelTooLowPopup => render_welcome_view(frame, app, &main_layout), // Render welcome view in background
        AppState::QuestionGeneration => render_question_generation_view(frame, app, &main_layout), // Reuse loading view for question generation
        AppState::QuestionAnswering => render_question_answering_view(frame, app, &main_layout),
        AppState::ApplicationGeneration => render_loading_view(frame, app, &main_layout), // Reuse loading view for application generation
        AppState::ApplicationDisplay => render_application_display_view(frame, app, &main_layout),
    }

    // Render modals over everything else
    if app.show_help {
        render_help_modal(frame);
    }
    if app.show_quit_confirmation {
        render_quit_modal(frame, app);
    }
    if let AppState::LevelTooLowPopup = app.current_state {
        render_level_too_low_popup(frame);
    }
}

pub fn render_question_generation_view(frame: &mut Frame, app: &App, layout: &[Rect]) {

    // Create a centered layout for the loading message
    let loading_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(4),
            Constraint::Percentage(40),
        ])
        .split(layout[1]);

    // Render loading message with animation
    let loading_text = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "<  Generating your learning module questions...  >",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    let loading = Paragraph::new(loading_text).alignment(Alignment::Center);

    frame.render_widget(loading, loading_layout[1]);

    // Render footer
    let status = Paragraph::new("Please wait...")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(status, layout[2]);
}
// Functions for rendering different views
pub fn render_welcome_view(frame: &mut Frame, app: &App, layout: &[Rect]) {
    // Render title bar
    let title = Paragraph::new("Rust AI Mentor v0.1.0")
        .style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(title, layout[0]);

    // Create a centered layout for the main content
    let main_content_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20), // Empty space above
            Constraint::Length(2),      // Welcome text
            Constraint::Length(1),      // Empty line
            Constraint::Length(10),     // Level selection (1-10)
            Constraint::Length(2),      // Prompt
            Constraint::Percentage(20), // Empty space below
        ])
        .split(layout[1]);

    // Render welcome text
    let welcome_text = Paragraph::new("Welcome to the Rust AI Mentor!")
        .alignment(Alignment::Center)
        .style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(welcome_text, main_content_layout[1]);

    // Create level selection list
    let mut level_lines = Vec::new();
    for level in 1..=10 {
        let line = if level == app.selected_level {
            // Selected level
            Line::from(vec![Span::styled(
                format!("> Level {}: {}", level, level_description(level)),
                Style::default().fg(Color::Black).bg(Color::LightYellow),
            )])
        } else {
            // Unselected level
            Line::from(vec![Span::raw(format!(
                "  Level {}: {}",
                level,
                level_description(level)
            ))])
        };
        level_lines.push(line);
    }

    let levels = Paragraph::new(level_lines).alignment(Alignment::Center);
    frame.render_widget(levels, main_content_layout[3]);

    // Render prompt
    let prompt = Paragraph::new("[ Press Enter to Begin ]")
        .alignment(Alignment::Center)
        .style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(prompt, main_content_layout[4]);

    // Render footer
    let footer_spans = vec![
        Span::raw("(k/↑, j/↓) Change Level | (?) Help "),
        Span::raw("| (s) Settings | (q) Quit"),
    ];

    let footer_line = Line::from(footer_spans);
    let status = Paragraph::new(footer_line)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(status, layout[2]);
}

// Helper function to get level descriptions
fn level_description(level: u8) -> &'static str {
    match level {
        1 => "Absolute Beginner",
        2 => "Beginner",
        3 => "Early Intermediate",
        4 => "Intermediate",
        5 => "Solid Intermediate",
        6 => "Advanced Intermediate",
        7 => "Early Advanced",
        8 => "Advanced",
        9 => "Very Advanced",
        10 => "Expert",
        _ => "Unknown",
    }
}


fn add_colors<'a>(highlighter: &mut HighlightLines, code: &'a str) -> Vec<Line<'a>> {
    let mut content_lines: Vec<Line<'a>> = Vec::new();

    // Split and wrap lines directly from code
    for line in code.split('\n') {
        let wrapped_lines = wrap(line, 78); // Returns Vec<Cow<'_, str>>
        for wrapped_line in wrapped_lines {
            // Convert the wrapped line to a String to extend its lifetime
            let wrapped_owned = wrapped_line.to_string();

            // Highlight the wrapped line
            let highlighted = highlighter
                .highlight_line(&wrapped_owned, &SYNTAX_SET)
                .unwrap_or_default();

            // Convert syntect styles to ratatui styles
            let mut spans = Vec::new();
            for (style, text) in highlighted {
                let fg_color = Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                let ratatui_style = Style::default().fg(fg_color);
                spans.push(Span::styled(text.to_string(), ratatui_style)); // Convert text to owned String
            }

            // Add the line with border
            let mut line_spans = vec![Span::raw("│ ")];
            line_spans.extend(spans);
            content_lines.push(Line::from(line_spans));
        }
    }

    content_lines
}
pub fn render_learning_view(frame: &mut Frame, app: &App, layout: &[Rect]) {
    // Add the code lines with syntax highlighting using syntect
    // Get the Rust syntax reference
    let syntax_ref = SYNTAX_SET
        .find_syntax_by_extension("rs")
        .unwrap_or_else(|| {
            SYNTAX_SET
                .find_syntax_by_name("Rust")
                .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text())
        });

    // Create a new highlighter with the Rust syntax and a theme
    let mut highlighter =
        HighlightLines::new(syntax_ref, &THEME_SET.themes["base16-ocean.dark"]);

    // Render title bar
    let title = Paragraph::new(format!("Rust AI Mentor :: Level {}", app.selected_level))
        .style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(title, layout[0]);

    // Check if we have a learning module to display
    if let Some(module) = &app.current_module {
        // Create a scrollable area for the content
        let mut content_lines = Vec::new();

        // Add topic header
        content_lines.push(Line::from(vec![Span::styled(
            format!("## TOPIC: {}", module.topic),
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )]));
        content_lines.push(Line::from(""));

        content_lines.append(&mut add_colors(&mut highlighter, &module.explanation));

        content_lines.push(Line::from(""));

        // Add code snippets with syntax highlighting
        for (i, snippet) in module.code_snippets.iter().enumerate() {
            content_lines.push(Line::from(vec![Span::styled(
                format!("{}: {}", snippet.title, i + 1),
                Style::default().add_modifier(Modifier::BOLD),
            )]));

            if !snippet.description.is_empty() {
                content_lines.push(Line::from(snippet.description.clone()));
            }

            // Add the code lines inside a block
            content_lines.push(Line::from(""));

            // Add a border line
            content_lines.push(Line::from(
                "┌─ Rust Code ───────────────────────────────────────────────────────────────────────┐",
            ));

            content_lines.append(&mut add_colors(&mut highlighter, &snippet.code));

            // Add a bottom border
            content_lines.push(Line::from(
                "└────────────────────────────────────────────────────────────────────────────────────┘",
            ));
            content_lines.push(Line::from(""));
        }

        // Add exercises with text wrapping
        content_lines.push(Line::from(vec![Span::styled(
            "Exercises:",
            Style::default().add_modifier(Modifier::BOLD),
        )]));

        for (i, exercise) in module.exercises.iter().enumerate() {
            content_lines.push(Line::from(vec![Span::styled(
                format!("Exercise {}: {}", i + 1, exercise.name),
                Style::default().add_modifier(Modifier::BOLD),
            )]));

            if !exercise.description.is_empty() {
                content_lines.push(Line::from(exercise.description.clone()));
            }

            content_lines.append(&mut add_colors(&mut highlighter, &exercise.code));

            content_lines.push(Line::from(""));
        }

        // Add additional learning resources if available
        if let Some(resources) = &module.additional_resources {
            content_lines.push(Line::from(""));
            content_lines.push(Line::from(vec![Span::styled(
                "Additional Learning Resources:",
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )]));
            content_lines.push(Line::from(""));

            // Official documentation
            if !resources.official_docs.is_empty() {
                content_lines.push(Line::from(vec![Span::styled(
                    "Official Documentation:",
                    Style::default().add_modifier(Modifier::BOLD),
                )]));

                for resource in &resources.official_docs {
                    content_lines.push(Line::from(vec![
                        Span::styled(format!("• {}: ", resource.title), Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(resource.url.clone(), Style::default().fg(Color::LightBlue)),
                    ]));
                    content_lines.push(Line::from(format!("  {}", resource.description)));
                }
                content_lines.push(Line::from(""));
            }

            // Community resources
            if !resources.community_resources.is_empty() {
                content_lines.push(Line::from(vec![Span::styled(
                    "Community Resources:",
                    Style::default().add_modifier(Modifier::BOLD),
                )]));

                for resource in &resources.community_resources {
                    content_lines.push(Line::from(vec![
                        Span::styled(format!("• {}: ", resource.title), Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(resource.url.clone(), Style::default().fg(Color::LightBlue)),
                    ]));
                    content_lines.push(Line::from(format!("  {}", resource.description)));
                }
                content_lines.push(Line::from(""));
            }

            // Crates.io links
            if !resources.crates_io.is_empty() {
                content_lines.push(Line::from(vec![Span::styled(
                    "Crates.io Packages:",
                    Style::default().add_modifier(Modifier::BOLD),
                )]));

                for resource in &resources.crates_io {
                    content_lines.push(Line::from(vec![
                        Span::styled(format!("• {}: ", resource.title), Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(resource.url.clone(), Style::default().fg(Color::LightBlue)),
                    ]));
                    content_lines.push(Line::from(format!("  {}", resource.description)));
                }
                content_lines.push(Line::from(""));
            }

            // GitHub repositories
            if !resources.github_repos.is_empty() {
                content_lines.push(Line::from(vec![Span::styled(
                    "GitHub Repositories:",
                    Style::default().add_modifier(Modifier::BOLD),
                )]));

                for resource in &resources.github_repos {
                    content_lines.push(Line::from(vec![
                        Span::styled(format!("• {}: ", resource.title), Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(resource.url.clone(), Style::default().fg(Color::LightBlue)),
                    ]));
                    content_lines.push(Line::from(format!("  {}", resource.description)));
                }
                content_lines.push(Line::from(""));
            }
        }

        // Create the scrollable paragraph
        let content = Paragraph::new(content_lines)
            .block(Block::default().borders(Borders::NONE))
            .scroll((app.scroll_offset, 0));

        frame.render_widget(content, layout[1]);
    } else {
        // If no module is loaded, show a placeholder
        let placeholder = Paragraph::new("No learning module loaded. Press 'n' to generate one.")
            .alignment(Alignment::Center);
        frame.render_widget(placeholder, layout[1]);
    }

    // Render footer
    let status = Paragraph::new("(n) New Module | (k/↑, j/↓) Scroll | (?) Help | (q) Quit")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(status, layout[2]);
}

pub fn render_index_selection_view(frame: &mut Frame, app: &App, layout: &[Rect]) {
    // Render title bar
    let title = Paragraph::new(format!("Rust AI Mentor :: Level {}", app.selected_level))
        .style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(title, layout[0]);

    // Create a centered layout for the index selection
    let selection_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(10), // Index selection options
            Constraint::Percentage(30),
        ])
        .split(layout[1]);

    // Create index selection list
    let mut index_lines = Vec::new();

    // Add title
    index_lines.push(Line::from(vec![Span::styled(
        "Select Learning Index:",
        Style::default().add_modifier(Modifier::BOLD),
    )]));
    index_lines.push(Line::from(""));

    // Add options
    let options = [
        "Rust Library Index (libraries like tokio, serde, etc.)",
        "Rust By Example Index (examples from Rust By Example)",
        "Rust Programming Language Index (topics from The Book)",
        "Random (select randomly from available indexes)",
    ];

    for (i, option) in options.iter().enumerate() {
        let line = if i == app.index_selection_cursor {
            // Selected option
            Line::from(vec![Span::styled(
                format!("> {}", option),
                Style::default().fg(Color::Black).bg(Color::LightYellow),
            )])
        } else {
            // Unselected option
            Line::from(vec![Span::raw(format!("  {}", option))])
        };
        index_lines.push(line);
    }

    let indexes = Paragraph::new(index_lines).alignment(Alignment::Center);
    frame.render_widget(indexes, selection_layout[1]);

    // Render footer
    let footer_spans = vec![
        Span::raw("(k/↑, j/↓) Change Selection | (Enter) Confirm | (Esc) Back "),
        Span::raw("| (?) Help | (s) Settings | (q) Quit"),
    ];

    let footer_line = Line::from(footer_spans);
    let status = Paragraph::new(footer_line)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(status, layout[2]);
}

pub fn render_loading_view(frame: &mut Frame, app: &App, layout: &[Rect]) {
    // Render title bar (same as learning view)
    let title = Paragraph::new(format!("Rust AI Mentor :: Level {}", app.selected_level))
        .style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(title, layout[0]);

    // Create a centered layout for the loading message
    let loading_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(4),
            Constraint::Percentage(40),
        ])
        .split(layout[1]);

    // Render loading message with animation
    let loading_text = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "<  Generating your learning module...  >",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    let loading = Paragraph::new(loading_text).alignment(Alignment::Center);

    frame.render_widget(loading, loading_layout[1]);

    // Render footer
    let status = Paragraph::new("Please wait...")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(status, layout[2]);
}

// Helper function for rendering modals
fn render_modal(frame: &mut Frame, area: Rect, widget: impl Widget) {
    // Clear the background
    frame.render_widget(Clear, area);
    // Render the widget
    frame.render_widget(widget, area);
}

pub fn render_settings_view(frame: &mut Frame, app: &App, layout: &[Rect]) {
    // Render title bar
    let title = Paragraph::new("Rust AI Mentor :: Settings")
        .style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(title, layout[0]);

    // Create a layout for the settings content
    let settings_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Left sidebar (sections)
            Constraint::Percentage(70), // Right content (options)
        ])
        .split(layout[1]);

    // Render sections sidebar
    let sections = vec![
        "Learning Resources",
        "Content Customization",
        "Learning Goals",
        "Question Generator",
    ];

    let mut section_lines = Vec::new();
    for (i, section) in sections.iter().enumerate() {
        let is_selected = match (i, &app.settings_section) {
            (0, SettingsSection::LearningResources) => true,
            (1, SettingsSection::ContentCustomization) => true,
            (2, SettingsSection::LearningGoals) => true,
            (3, SettingsSection::QuestionGenerator) => true,
            _ => false,
        };

        let line = if is_selected {
            Line::from(vec![Span::styled(
                format!("> {}", section),
                Style::default().fg(Color::Black).bg(Color::LightYellow),
            )])
        } else {
            Line::from(vec![Span::raw(format!("  {}", section))])
        };
        section_lines.push(line);
    }

    let sections_widget = Paragraph::new(section_lines)
        .block(Block::default().borders(Borders::RIGHT).title("Sections"));
    frame.render_widget(sections_widget, settings_layout[0]);

    // Render options based on selected section
    match app.settings_section {
        SettingsSection::LearningResources => {
            render_learning_resources_settings(frame, app, settings_layout[1]);
        }
        SettingsSection::ContentCustomization => {
            render_content_customization_settings(frame, app, settings_layout[1]);
        }
        SettingsSection::LearningGoals => {
            render_learning_goals_settings(frame, app, settings_layout[1]);
        }
        SettingsSection::QuestionGenerator => {
            render_question_generator_settings(frame, app, settings_layout[1]);
        }
    }

    // Render footer
    let status = Paragraph::new("(Tab) Switch Section | (k/↑, j/↓) Navigate | (Enter/Space) Toggle | (Esc) Back | (?) Help")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(status, layout[2]);
}

fn render_learning_resources_settings(frame: &mut Frame, app: &App, area: Rect) {
    let resources = app.get_learning_resources();

    let mut option_lines = Vec::new();
    option_lines.push(Line::from(vec![Span::styled(
        "Learning Resources Settings",
        Style::default().add_modifier(Modifier::BOLD),
    )]));
    option_lines.push(Line::from(""));

    // Show Official Documentation
    let official_docs_line = if app.settings_cursor == 0 {
        Line::from(vec![Span::styled(
            format!("> Show Official Documentation: [{}]", if resources.show_official_docs { "X" } else { " " }),
            Style::default().fg(Color::Black).bg(Color::LightYellow),
        )])
    } else {
        Line::from(vec![Span::raw(
            format!("  Show Official Documentation: [{}]", if resources.show_official_docs { "X" } else { " " }),
        )])
    };
    option_lines.push(official_docs_line);

    // Show Community Resources
    let community_line = if app.settings_cursor == 1 {
        Line::from(vec![Span::styled(
            format!("> Show Community Resources: [{}]", if resources.show_community_resources { "X" } else { " " }),
            Style::default().fg(Color::Black).bg(Color::LightYellow),
        )])
    } else {
        Line::from(vec![Span::raw(
            format!("  Show Community Resources: [{}]", if resources.show_community_resources { "X" } else { " " }),
        )])
    };
    option_lines.push(community_line);

    // Show Crates.io Links
    let crates_line = if app.settings_cursor == 2 {
        Line::from(vec![Span::styled(
            format!("> Show Crates.io Links: [{}]", if resources.show_crates_io { "X" } else { " " }),
            Style::default().fg(Color::Black).bg(Color::LightYellow),
        )])
    } else {
        Line::from(vec![Span::raw(
            format!("  Show Crates.io Links: [{}]", if resources.show_crates_io { "X" } else { " " }),
        )])
    };
    option_lines.push(crates_line);

    // Show GitHub Repositories
    let github_line = if app.settings_cursor == 3 {
        Line::from(vec![Span::styled(
            format!("> Show GitHub Repositories: [{}]", if resources.show_github_repos { "X" } else { " " }),
            Style::default().fg(Color::Black).bg(Color::LightYellow),
        )])
    } else {
        Line::from(vec![Span::raw(
            format!("  Show GitHub Repositories: [{}]", if resources.show_github_repos { "X" } else { " " }),
        )])
    };
    option_lines.push(github_line);

    option_lines.push(Line::from(""));
    option_lines.push(Line::from("These settings control what additional learning resources are shown alongside the AI-generated content."));

    let options_widget = Paragraph::new(option_lines)
        .block(Block::default().borders(Borders::NONE).title("Options"));
    frame.render_widget(options_widget, area);
}

fn render_content_customization_settings(frame: &mut Frame, app: &App, area: Rect) {
    let customization = app.get_content_customization();

    let mut option_lines = Vec::new();
    option_lines.push(Line::from(vec![Span::styled(
        "Content Customization Settings",
        Style::default().add_modifier(Modifier::BOLD),
    )]));
    option_lines.push(Line::from(""));

    // Code Complexity
    let complexity_text = match customization.code_complexity {
        CodeComplexity::Simple => "Simple",
        CodeComplexity::Moderate => "Moderate",
        CodeComplexity::Complex => "Complex",
    };

    let complexity_line = if app.settings_cursor == 0 {
        Line::from(vec![Span::styled(
            format!("> Code Complexity: [{}]", complexity_text),
            Style::default().fg(Color::Black).bg(Color::LightYellow),
        )])
    } else {
        Line::from(vec![Span::raw(
            format!("  Code Complexity: [{}]", complexity_text),
        )])
    };
    option_lines.push(complexity_line);

    // Explanation Verbosity
    let verbosity_text = match customization.explanation_verbosity {
        ExplanationVerbosity::Concise => "Concise",
        ExplanationVerbosity::Moderate => "Moderate",
        ExplanationVerbosity::Detailed => "Detailed",
    };

    let verbosity_line = if app.settings_cursor == 1 {
        Line::from(vec![Span::styled(
            format!("> Explanation Verbosity: [{}]", verbosity_text),
            Style::default().fg(Color::Black).bg(Color::LightYellow),
        )])
    } else {
        Line::from(vec![Span::raw(
            format!("  Explanation Verbosity: [{}]", verbosity_text),
        )])
    };
    option_lines.push(verbosity_line);

    // Focus Area
    let focus_text = match customization.focus_area {
        FocusArea::Concepts => "Concepts",
        FocusArea::CodeExamples => "Code Examples",
        FocusArea::Exercises => "Exercises",
        FocusArea::Balanced => "Balanced",
    };

    let focus_line = if app.settings_cursor == 2 {
        Line::from(vec![Span::styled(
            format!("> Focus Area: [{}]", focus_text),
            Style::default().fg(Color::Black).bg(Color::LightYellow),
        )])
    } else {
        Line::from(vec![Span::raw(
            format!("  Focus Area: [{}]", focus_text),
        )])
    };
    option_lines.push(focus_line);

    option_lines.push(Line::from(""));
    option_lines.push(Line::from("These settings control how the AI generates content for your learning modules."));

    let options_widget = Paragraph::new(option_lines)
        .block(Block::default().borders(Borders::NONE).title("Options"));
    frame.render_widget(options_widget, area);
}
fn render_question_generator_settings(frame: &mut Frame, app: &App, area: Rect) {
    let customization = app.get_question_generator_settings();

    let mut option_lines = Vec::new();
    option_lines.push(Line::from(vec![Span::styled(
        "Question Generator Settings",
        Style::default().add_modifier(Modifier::BOLD),
    )]));
    option_lines.push(Line::from(""));

    // Number of Questions
    let num_questions_line = if app.settings_cursor == 0 {
        Line::from(vec![Span::styled(
            format!("> Number of Questions: [{}]", customization.num_questions),
            Style::default().fg(Color::Black).bg(Color::LightYellow),
        )])
    } else {
        Line::from(vec![Span::raw(
            format!("  Number of Questions: [{}]", customization.num_questions),
        )])
    };

    option_lines.push(num_questions_line);

    // Question Type
    let question_type_line = if app.settings_cursor == 1 {
        Line::from(vec![Span::styled(
            format!("> Question Type: [{}]", customization.default_question_type),
            Style::default().fg(Color::Black).bg(Color::LightYellow),
        )])
    } else {
        Line::from(vec![Span::raw(
            format!("  Question Type: [{}]", customization.default_question_type),
        )])
    };
    option_lines.push(question_type_line);

    option_lines.push(Line::from(""));
    option_lines.push(Line::from("These settings control how questions are generated for testing your understanding."));

    let options_widget = Paragraph::new(option_lines)
        .block(Block::default().borders(Borders::NONE).title("Options"));
    frame.render_widget(options_widget, area);
}
fn render_learning_goals_settings(frame: &mut Frame, app: &App, area: Rect) {
        let learning_goal = app.get_learning_goal();
        let goal_text = learning_goal.to_string();

        let mut option_lines = Vec::new();
        option_lines.push(Line::from(vec![Span::styled(
            "Learning Goals Settings",
            Style::default().add_modifier(Modifier::BOLD),
        )]));
        option_lines.push(Line::from(""));

        // Learning Goal
        let goal_line = if app.settings_cursor == 0 {
            Line::from(vec![Span::styled(
                format!("> Learning Goal: [{}]", goal_text),
                Style::default().fg(Color::Black).bg(Color::LightYellow),
            )])
        } else {
            Line::from(vec![Span::raw(
                format!("  Learning Goal: [{}]", goal_text),
            )])
        };
        option_lines.push(goal_line);

        option_lines.push(Line::from(""));
        option_lines.push(Line::from("These settings control the focus of your learning path in Rust."));

        let options_widget = Paragraph::new(option_lines)
            .block(Block::default().borders(Borders::NONE).title("Options"));
        frame.render_widget(options_widget, area);
    }

    pub fn render_help_modal(frame: &mut Frame) {
        // Calculate a centered rect for the modal
        let area = centered_rect(60, 60, frame.size());

        // Create the help content
        let help_text = vec![
            Line::from("Global Keybindings:"),
            Line::from("  ? - Toggle help"),
            Line::from("  q - Quit"),
            Line::from("  s - Open settings"),
            Line::from(""),
            Line::from("Welcome Screen:"),
            Line::from("  k/↑, j/↓ - Change level"),
            Line::from("  Enter - Proceed to index selection"),
            Line::from(""),
            Line::from("Index Selection Screen:"),
            Line::from("  k/↑, j/↓ - Change selection"),
            Line::from("  Enter - Confirm selection and generate module"),
            Line::from("  Esc - Return to welcome screen"),
            Line::from(""),
            Line::from("Learning Screen:"),
            Line::from("  k/↑, j/↓ - Scroll content"),
            Line::from("  n - Request new module"),
            Line::from("  Esc - Return to welcome screen"),
            Line::from(""),
            Line::from("Settings Screen:"),
            Line::from("  Tab - Switch between sections"),
            Line::from("  k/↑, j/↓ - Navigate options"),
            Line::from("  Enter/Space - Toggle or cycle selected option"),
            Line::from("  Esc - Return to welcome screen"),
        ];

        let help_content = Paragraph::new(help_text).block(
            Block::default()
                .title("Keybindings")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

        render_modal(frame, area, help_content);
    }

    pub fn render_quit_modal(frame: &mut Frame, app: &App) {
        // Calculate a small centered rect for the modal
        let area = centered_rect(40, 20, frame.size());

        // Create the quit confirmation content
        let options = Line::from(vec![
            Span::raw("[ "),
            Span::styled(
                "Yes",
                Style::default().add_modifier(if app.quit_confirmation_selected {
                    Modifier::REVERSED
                } else {
                    Modifier::empty()
                }),
            ),
            Span::raw(" ] [ "),
            Span::styled(
                "No",
                Style::default().add_modifier(if !app.quit_confirmation_selected {
                    Modifier::REVERSED
                } else {
                    Modifier::empty()
                }),
            ),
            Span::raw(" ]"),
        ]);

        let quit_content = Paragraph::new(vec![
            Line::from("Are you sure you want to quit?"),
            Line::from(""),
            options,
        ])
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );

        render_modal(frame, area, quit_content);
    }

    pub fn render_level_too_low_popup(frame: &mut Frame) {
        // Calculate a small centered rect for the modal
        let area = centered_rect(60, 20, frame.size());

        // Create the level too low content
        let content = Paragraph::new(vec![
            Line::from("For library subject - you need to be a level 3 programmer (or higher)"),
            Line::from(""),
            Line::from("Returning to programmer level selection..."),
        ])
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Level Too Low")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );

        render_modal(frame, area, content);
    }

    // Helper function to create a centered rect
    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }

// Render the question answering view
pub fn render_question_answering_view(frame: &mut Frame, app: &App, layout: &[Rect]) {
    // Render title bar
    let title = Paragraph::new("Rust AI Mentor - Question Answering")
        .style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(title, layout[0]);

    // Check if we have a question set
    if let Some(question_set) = &app.question_set {
        // Create a layout for the main content
        let main_content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),  // Topic
                Constraint::Length(1),  // Empty line
                Constraint::Length(2),  // Progress
                Constraint::Length(1),  // Empty line
                Constraint::Min(5),     // Question
                Constraint::Length(1),  // Empty line
                Constraint::Length(6),  // Answer options
                Constraint::Min(0),     // Empty space
            ])
            .split(layout[1]);

        // Render topic
        let topic_text = format!("Topic: {}", question_set.topic);
        let topic = Paragraph::new(topic_text)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        frame.render_widget(topic, main_content_layout[0]);

        // Render progress
        let (answered, total) = question_set.progress();
        let progress_text = format!("Question {}/{}", question_set.current_question_index + 1, total);
        let progress_bar_width = main_content_layout[2].width as usize - 4;
        let filled_width = if total > 0 {
            (progress_bar_width * answered) / total
        } else {
            0
        };

        let progress_bar = format!(
            "[{}{}] {}/{} answered",
            "=".repeat(filled_width),
            " ".repeat(progress_bar_width - filled_width),
            answered,
            total
        );

        let progress = Paragraph::new(progress_bar)
            .alignment(Alignment::Center);
        frame.render_widget(progress, main_content_layout[2]);

        // Render current question
        if let Some(current_question) = question_set.current_question() {
            let question_text = format!("Q: {}", current_question.text);
            let wrapped_text = textwrap::wrap(&question_text, main_content_layout[4].width as usize - 4)
                .iter()
                .map(|line| Line::from(line.to_string()))
                .collect::<Vec<_>>();

            let question = Paragraph::new(wrapped_text)
                .block(Block::default().borders(Borders::ALL).title("Question"));
            frame.render_widget(question, main_content_layout[4]);

            // Render answer options
            let mut option_lines = Vec::new();

            match current_question.question_type {
                crate::question_generator::QuestionType::Binary => {
                    let yes_style = if current_question.selected_answer.as_deref() == Some("Yes") {
                        Style::default().fg(Color::Black).bg(Color::LightYellow)
                    } else {
                        Style::default()
                    };

                    let no_style = if current_question.selected_answer.as_deref() == Some("No") {
                        Style::default().fg(Color::Black).bg(Color::LightYellow)
                    } else {
                        Style::default()
                    };

                    option_lines.push(Line::from(vec![
                        Span::styled("(Y) Yes", yes_style),
                        Span::raw("   "),
                        Span::styled("(N) No", no_style),
                    ]));
                },
                crate::question_generator::QuestionType::Multiple => {
                    for option in &current_question.options {
                        let style = if current_question.selected_answer.as_deref() == Some(&option.id) {
                            Style::default().fg(Color::Black).bg(Color::LightYellow)
                        } else {
                            Style::default()
                        };

                        option_lines.push(Line::from(vec![
                            Span::styled(format!("({}) {}", option.id, option.text), style),
                        ]));
                    }
                }
            }

            let options = Paragraph::new(option_lines)
                .block(Block::default().borders(Borders::ALL).title("Answer Options"));
            frame.render_widget(options, main_content_layout[6]);
        }
    } else {
        // No question set available
        let message = Paragraph::new("No questions available. Press 'q' in Learning mode to generate questions.")
            .alignment(Alignment::Center)
            .style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(message, layout[1]);
    }

    // Render footer
    let footer_spans = vec![
        Span::raw("(←/→) Navigate Questions | (Enter) Submit Answers "),
        Span::raw("| (Esc) Back to Learning"),
    ];

    let footer_line = Line::from(footer_spans);
    let status = Paragraph::new(footer_line)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(status, layout[2]);
}

// Render the application display view
pub fn render_application_display_view(frame: &mut Frame, app: &App, layout: &[Rect]) {
    // Render title bar
    let title = Paragraph::new("Rust AI Mentor - Generated Application")
        .style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(title, layout[0]);

    // Check if we have a generated application
    if let Some(application) = &app.generated_application {
        // Create a layout for the main content
        let main_content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),  // Application name
                Constraint::Length(1),  // Empty line
                Constraint::Length(4),  // Description
                Constraint::Length(1),  // Empty line
                Constraint::Length(6),  // Features
                Constraint::Length(1),  // Empty line
                Constraint::Min(10),    // Code snippets
            ])
            .split(layout[1]);

        // Render application name
        let name_text = format!("Application: {}", application.name);
        let name = Paragraph::new(name_text)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        frame.render_widget(name, main_content_layout[0]);

        // Render description
        let wrapped_description = textwrap::wrap(&application.description, main_content_layout[2].width as usize - 4)
            .iter()
            .map(|line| Line::from(line.to_string()))
            .collect::<Vec<_>>();

        let description = Paragraph::new(wrapped_description)
            .block(Block::default().borders(Borders::ALL).title("Description"));
        frame.render_widget(description, main_content_layout[2]);

        // Render features
        let feature_lines: Vec<Line> = application.features
            .iter()
            .map(|feature| Line::from(format!("• {}", feature)))
            .collect();

        let features = Paragraph::new(feature_lines)
            .block(Block::default().borders(Borders::ALL).title("Features"));
        frame.render_widget(features, main_content_layout[4]);

        // Render code snippets (just the first one for now)
        if let Some(first_snippet) = application.code_snippets.first() {
            let syntax = SYNTAX_SET.find_syntax_by_extension("rs").unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());
            let mut highlighter = HighlightLines::new(syntax, &THEME_SET.themes["base16-ocean.dark"]);

            let highlighted_code = add_colors(&mut highlighter, &first_snippet.code);

            let code = Paragraph::new(highlighted_code)
                .block(Block::default().borders(Borders::ALL).title(format!("Code: {}", first_snippet.title)));
            frame.render_widget(code, main_content_layout[6]);
        }
    } else {
        // No application available
        let message = Paragraph::new("No application generated yet. Answer all questions to generate an application.")
            .alignment(Alignment::Center)
            .style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(message, layout[1]);
    }

    // Render footer
    let footer_spans = vec![
        Span::raw("(Enter) Create Project | (Esc) Back to Learning"),
    ];

    let footer_line = Line::from(footer_spans);
    let status = Paragraph::new(footer_line)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(status, layout[2]);
}
