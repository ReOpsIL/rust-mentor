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
    ];

    let mut section_lines = Vec::new();
    for (i, section) in sections.iter().enumerate() {
        let is_selected = match (i, &app.settings_section) {
            (0, SettingsSection::LearningResources) => true,
            (1, SettingsSection::ContentCustomization) => true,
            (2, SettingsSection::LearningGoals) => true,
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

