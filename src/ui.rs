// src/ui.rs
use crate::app::{App, AppState};
use lazy_static::lazy_static;
use ratatui::prelude::*;
use ratatui::widgets::*;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use textwrap;

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
        AppState::Learning => render_learning_view(frame, app, &main_layout),
        AppState::Loading => render_loading_view(frame, app, &main_layout),
    }

    // Render modals over everything else
    if app.show_help {
        render_help_modal(frame);
    }
    if app.show_quit_confirmation {
        render_quit_modal(frame, app);
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
        Span::raw("(k/↑, j/↓) Change Level | (?) Help"),
        Span::raw("(q) Quit"),
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

pub fn render_learning_view(frame: &mut Frame, app: &App, layout: &[Rect]) {
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

        // Add explanation text (split by newlines and wrap long lines)
        for line in module.explanation.split("\\n") {
            // Wrap lines longer than 80 characters
            let wrapped_lines = textwrap::wrap(line, 80);
            for wrapped_line in wrapped_lines {
                content_lines.push(Line::from(wrapped_line.to_string()));
            }
        }
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
                "┌─ Rust Code ─────────────────────────────────────────────────────────┐",
            ));
            
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
            
            // Process each line of the code snippet
            for line in LinesWithEndings::from(&snippet.code) {
                // Highlight the line
                let highlighted = highlighter
                    .highlight_line(line, &SYNTAX_SET)
                    .unwrap_or_default();
                
                // Convert syntect styles to ratatui styles and create spans
                let mut spans = Vec::new();
                for (style, text) in highlighted {
                    // Convert to a ratatui Color
                    let fg_color =
                        Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                    
                    let ratatui_style = Style::default().fg(fg_color);
                    spans.push(Span::styled(text, ratatui_style));
                }
                
                // Add the line with border
                let mut line_spans = vec![Span::raw("│ ")];
                line_spans.extend(spans);
                content_lines.push(Line::from(line_spans));
            }
            
            // Add a bottom border
            content_lines.push(Line::from(
                "└──────────────────────────────────────────────────────────────────────┘",
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
            
            // Wrap exercise code text
            for exercise_line in exercise.code.split('\n') {
                let wrapped_lines = textwrap::wrap(exercise_line, 78);
                
                // First line
                if let Some(first_line) = wrapped_lines.first() {
                    content_lines.push(Line::from(first_line.to_string()));
                }
                
                // Subsequent lines are indented
                for line in wrapped_lines.iter().skip(1) {
                    content_lines.push(Line::from(format!("   {}", line)));
                }
            }
            
            content_lines.push(Line::from(""));
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

pub fn render_help_modal(frame: &mut Frame) {
    // Calculate a centered rect for the modal
    let area = centered_rect(60, 60, frame.size());

    // Create the help content
    let help_text = vec![
        Line::from("Global Keybindings:"),
        Line::from("  ? - Toggle help"),
        Line::from("  q - Quit"),
        Line::from(""),
        Line::from("Welcome Screen:"),
        Line::from("  k/↑, j/↓ - Change level"),
        Line::from("  Enter - Start learning"),
        Line::from(""),
        Line::from("Learning Screen:"),
        Line::from("  k/↑, j/↓ - Scroll content"),
        Line::from("  n - Request new module"),
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
