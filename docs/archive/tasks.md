### **Phase 0: Project Setup & Initial Configuration**
*   **Goal:** Create the project structure, initialize version control, and prepare the necessary configuration and data files according to `implementation.md`.

1.  **[Task 0.1]** In your terminal, execute `cargo new --bin rust-ai-mentor` to create a new Rust binary project.
2.  **[Task 0.2]** Navigate into the new directory: `cd rust-ai-mentor`.
3.  **[Task 0.3]** Initialize a Git repository: `git init`.
4.  **[Task 0.4]** Create a `.gitignore` file and populate it with the following content to ignore build artifacts, environment files, and IDE metadata:
    ```
    # Rust
    /target
    Cargo.lock

    # Environment
    .env
    .env.*
    !.env.example

    # IDEs
    .vscode/
    .idea/
    ```
5.  **[Task 0.5]** Create the required directory structure as defined in `implementation.md`:
    *   `mkdir -p src/components`
    *   `mkdir data`
6.  **[Task 0.6]** Create the empty module file for UI components: `touch src/components/mod.rs`.
7.  **[Task 0.7]** Create the following data files:

    a. `data/rust_library_index.json` - For Rust standard and community libraries
    *Example content for `data/rust_library_index.json`*:
    ```json
    [
      { "library_name": "std::collections::HashMap", "description": "A hash map implementation", "programmer_level": 3 },
      { "library_name": "std::vec::Vec", "description": "A contiguous growable array type", "programmer_level": 1 },
      { "library_name": "std::string::String", "description": "A UTF-8 encoded, growable string", "programmer_level": 1 },
      { "library_name": "std::option::Option", "description": "Type representing optional values", "programmer_level": 2 },
      { "library_name": "std::result::Result", "description": "Type for returning and propagating errors", "programmer_level": 3 }
    ]
    ```

    b. `data/rust_by_example_full.json` - For Rust By Example content
    *Example content for `data/rust_by_example_full.json`*:
    ```json
    {
      "book": {
        "chapters": [
          {
            "title": "Hello World",
            "sections": [
              {
                "title": "Comments",
                "section_number": "1.1",
                "min_level": 1
              },
              {
                "title": "Formatted Print",
                "section_number": "1.2",
                "min_level": 1
              }
            ]
          },
          {
            "title": "Primitives",
            "sections": [
              {
                "title": "Literals and Operators",
                "section_number": "2.1",
                "min_level": 1
              },
              {
                "title": "Tuples",
                "section_number": "2.2",
                "min_level": 2
              }
            ]
          }
        ]
      }
    }
    ```

    c. `data/the_rust_programming_language.json` - For The Rust Programming Language book
    *Example content for `data/the_rust_programming_language.json`*:
    ```json
    {
      "book": {
        "chapters": [
          {
            "title": "Getting Started",
            "sections": [
              {
                "title": "Installation",
                "section_number": "1.1",
                "min_level": 1
              },
              {
                "title": "Hello, World!",
                "section_number": "1.2",
                "min_level": 1
              }
            ]
          },
          {
            "title": "Common Programming Concepts",
            "sections": [
              {
                "title": "Variables and Mutability",
                "section_number": "3.1",
                "min_level": 1
              },
              {
                "title": "Data Types",
                "section_number": "3.2",
                "min_level": 1
              }
            ]
          }
        ]
      }
    }
    ```

    These data files support feature **F004** by providing multiple content sources for the application.
8.  **[Task 0.8]** Read OPENROUTER_API_KEY  environment variable  when the application loads.

---

### **Phase 1: Project Scaffolding & TUI Foundation**
*   **Goal:** Establish a runnable application shell with a working event loop that can initialize/restore the terminal and respond to a quit command. This phase builds the core of the **UI/Presentation Layer** and **Application Core** from the architecture diagram.

1.  **[Task 1.1]** Add core dependencies to `Cargo.toml`. These crates are chosen based on the "Technology Choices" in `architecture.md` and "Technology Stack" in `implementation.md`.
    ```toml
    [dependencies]
    anyhow = "1.0"
    crossterm = "0.27"
    ratatui = { version = "0.26", features = ["crossterm"] }
    tokio = { version = "1.36", features = ["full"] }
    tracing = "0.1"
    tracing-subscriber = "0.3"
    ```
2.  **[Task 1.2]** Create the initial set of module files inside the `src/` directory.
    *   `touch src/app.rs`
    *   `touch src/event.rs`
    *   `touch src/tui.rs`
    *   `touch src/ui.rs`
3.  **[Task 1.3]** In `src/main.rs`, declare the modules and set up the main entry point.
    ```rust
    // src/main.rs
    mod app;
    mod components;
    mod data;
    mod event;
    mod llm;
    mod tui;
    mod ui;
    mod prompt_response;
    mod config;
    mod cargo_project;

    use anyhow::Result;
    use app::App;
    use event::{Event, EventHandler};
    use tui::Tui;

    #[tokio::main]
    async fn main() -> Result<()> {
        // Initialize logging
        tracing_subscriber::fmt::init();

        // Create the application state
        let mut app = App::new();

        // Initialize the terminal user interface
        let mut tui = Tui::new()?;
        tui.enter()?;

        // Create an event handler
        let event_handler = EventHandler::new(250); // 250ms tick rate

        // Start the main loop
        while app.is_running {
            // Render the UI
            tui.draw(&mut app)?;
            // Handle events
            match event_handler.next().await? {
                Event::Tick => app.tick(),
                Event::Key(key_event) => app.handle_key_event(key_event)?,
            }
        }

        // Restore the terminal
        tui.exit()?;
        Ok(())
    }
    ```
4.  **[Task 1.4]** In `src/tui.rs`, implement the terminal handler. This component is responsible for low-level terminal operations.
    ```rust
    // src/tui.rs
    use anyhow::Result;
    use crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{backend::CrosstermBackend, Terminal};
    use std::io::{stdout, Stdout};
    use crate::{app::App, ui};

    pub struct Tui {
        terminal: Terminal<CrosstermBackend<Stdout>>,
    }

    impl Tui {
        pub fn new() -> Result<Self> {
            let backend = CrosstermBackend::new(stdout());
            let terminal = Terminal::new(backend)?;
            Ok(Self { terminal })
        }

        pub fn enter(&mut self) -> Result<()> {
            enable_raw_mode()?;
            execute!(stdout(), EnterAlternateScreen)?;
            Ok(())
        }

        pub fn exit(&mut self) -> Result<()> {
            disable_raw_mode()?;
            execute!(stdout(), LeaveAlternateScreen)?;
            Ok(())
        }

        pub fn draw(&mut self, app: &mut App) -> Result<()> {
            self.terminal.draw(|frame| ui::render(app, frame))?;
            Ok(())
        }
    }
    ```
5.  **[Task 1.5]** In `src/event.rs`, define the `Event` enum and `EventHandler` struct to manage input and ticks asynchronously, preventing UI blocking.
    ```rust
    // src/event.rs
    use anyhow::Result;
    use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, KeyCode};
    use std::time::Duration;
    use tokio::sync::mpsc;

    #[derive(Debug)]
    pub enum Event {
        Tick,
        Key(KeyEvent),
    }

    pub struct EventHandler {
        sender: mpsc::Sender<Event>,
        receiver: mpsc::Receiver<Event>,
    }

    impl EventHandler {
        pub fn new(tick_rate_ms: u64) -> Self {
            let (sender, receiver) = mpsc::channel(100);
            let tick_rate = Duration::from_millis(tick_rate_ms);
            let event_sender = sender.clone();

            tokio::spawn(async move {
                loop {
                    if event::poll(tick_rate).unwrap_or(false) {
                        if let Ok(CrosstermEvent::Key(key)) = event::read() {
                            event_sender.send(Event::Key(key)).await.ok();
                        }
                    }
                    event_sender.send(Event::Tick).await.ok();
                }
            });

            Self { sender, receiver }
        }

        pub async fn next(&mut self) -> Result<Event> {
            self.receiver.recv().await.ok_or_else(|| anyhow::anyhow!("Event channel closed"))
        }
    }
    ```
6.  **[Task 1.6]** In `src/app.rs`, define the initial `App` struct and its basic event handling logic. This is the **Application Core**.
    ```rust
    // src/app.rs
    use anyhow::Result;
    use crossterm::event::{KeyEvent, KeyCode};

    pub struct App {
        pub is_running: bool,
    }

    impl App {
        pub fn new() -> Self {
            Self { is_running: true }
        }

        pub fn tick(&mut self) {}

        pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
            if let KeyCode::Char('q') = key_event.code {
                self.is_running = false;
            }
            Ok(())
        }
    }
    ```
7.  **[Task 1.7]** In `src/ui.rs`, create the placeholder `render` function (**F001** partial).
    ```rust
    // src/ui.rs
    use ratatui::{
        prelude::{Alignment, Frame},
        widgets::{Block, Borders, Paragraph},
    };
    use crate::app::App;

    pub fn render(app: &mut App, frame: &mut Frame) {
        frame.render_widget(
            Paragraph::new("Welcome to Rust AI Mentor!\nPress 'q' to quit.")
                .block(Block::default().title("Rust AI Mentor").borders(Borders::ALL))
                .alignment(Alignment::Center),
            frame.size(),
        );
    }
    ```
8.  **[Task 1.8]** Run `cargo run`. The application should launch into a TUI with a border and centered text. Pressing 'q' should exit cleanly. This completes Phase 1.

---

### **Phase 2: Static UI Implementation & State-Based Navigation**
*   **Goal:** Build all TUI views and modals (**F001**, **F002**, **F005**) using static data and implement the navigation state machine, as described in `ux_specification.md`.

1.  **[Task 2.1]** In `src/app.rs`, expand the `App` struct to include state management.
    ```rust
    // In src/app.rs, update the App struct and add AppState
    pub enum AppState {
        Welcome,
        IndexSelection,
        Learning,
        Loading,
        LevelTooLowPopup,
    }

    pub struct App {
        pub is_running: bool,
        pub current_state: AppState,
        pub selected_level: u8,
        pub show_help: bool,
        pub show_quit_confirmation: bool,
    }

    impl App {
        pub fn new() -> Self {
            Self {
                is_running: true,
                current_state: AppState::Welcome,
                selected_level: 5, // Default level
                show_help: false,
                show_quit_confirmation: false,
            }
        }
        // ... (tick method remains)
    }
    ```
2.  **[Task 2.2]** In `src/app.rs`, update `handle_key_event` to manage state transitions based on the current state.
    ```rust
    // In src/app.rs, inside impl App
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        // Global keybindings
        if self.show_help {
            if let KeyCode::Esc | KeyCode::Char('?') = key_event.code {
                self.show_help = false;
            }
            return Ok(());
        }

        if self.show_quit_confirmation {
            match key_event.code {
                KeyCode::Enter => self.is_running = false,
                KeyCode::Esc | KeyCode::Char('q') => self.show_quit_confirmation = false,
                _ => {}
            }
            return Ok(());
        }

        match key_event.code {
            KeyCode::Char('q') => self.show_quit_confirmation = true,
            KeyCode::Char('?') => self.show_help = true,
            _ => {}
        }

        // Context-specific keybindings
        match self.current_state {
            AppState::Welcome => self.handle_welcome_keys(key_event),
            AppState::Learning => self.handle_learning_keys(key_event),
            _ => {}
        }
        Ok(())
    }

    // Add these new methods to impl App
    fn handle_welcome_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Right | KeyCode::Char('l') => {
                self.selected_level = (self.selected_level + 1).min(10);
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.selected_level = (self.selected_level - 1).max(1);
            }
            KeyCode::Enter => {
                self.current_state = AppState::Loading;
                // In Phase 3, we will trigger the LLM fetch here.
                // For now, we'll just switch to the Learning view with mock data.
                self.current_state = AppState::Learning;
            }
            _ => {}
        }
    }

    fn handle_learning_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('n') => {
                self.current_state = AppState::Loading;
                 // For now, we'll just switch back to Learning.
                self.current_state = AppState::Learning;
            }
            _ => {}
        }
    }
    ```
3.  **[Task 2.3]** In `src/ui.rs`, update the main `render` function to delegate based on the application state. Add necessary imports.
    ```rust
    // src/ui.rs
    // ... imports
    use ratatui::prelude::*;
    use ratatui::widgets::*;
    use crate::app::{App, AppState};

    pub fn render(app: &mut App, frame: &mut Frame) {
        // Main layout
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)])
            .split(frame.size());

        render_title_bar(app, frame, main_layout[0]);
        render_status_bar(app, frame, main_layout[2]);

        // Render main content based on state
        match app.current_state {
            AppState::Welcome => render_welcome(app, frame, main_layout[1]),
            AppState::Learning => render_learning(app, frame, main_layout[1]),
            AppState::Loading => render_loading(app, frame, main_layout[1]),
        }

        // Render modals over everything else
        if app.show_help {
            render_help_modal(frame);
        }
        if app.show_quit_confirmation {
            render_quit_modal(frame);
        }
    }
    ```
4.  **[Task 2.4]** In `src/ui.rs`, create the static rendering functions for all views and modals, using `ratatui` widgets and layouts as specified in `ux_specification.md`.
    *   **`render_title_bar`**: Renders `Rust AI Mentor` and current level.
    *   **`render_status_bar`**: Renders contextual keybindings.
    *   **`render_welcome`**: Renders the welcome message and a level selector widget (Wireframe 1). Use a simple text-based representation for the selector.
    *   **`render_learning`**: Renders a mock learning module with placeholders for Topic, Explanation, Code, and Exercises (Wireframe 2). Use `Block`, `Paragraph`, and `Layout` to structure the content.
    *   **`render_loading`**: Renders a simple "Loading..." message.
    *   **`render_help_modal`**: Renders a centered modal with keybindings (Wireframe 3). Use a helper function `centered_rect` to calculate the modal's area, `Clear` widget to erase the background, then render the modal.
    *   **`render_quit_modal`**: Renders a "Are you sure you want to quit?" modal.

---

### **Phase 3: LLM Integration & Dynamic Content**
*   **Goal:** Replace mock data with live content from the OpenRouter API (**F003**, **F004**), fulfilling the core value proposition. This connects the **Application Core** to the **External Service Client** and **Data Access Layer**.

1.  **[Task 3.1]** Add new dependencies to `Cargo.toml`:
    ```toml
    [dependencies]
    # ... (existing dependencies)
    dotenvy = "0.15" # To load .env file
    reqwest = { version = "0.11", features = ["json"] }
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    ```
2.  **[Task 3.2]** Create `src/llm.rs` and `src/data.rs`. Declare them in `main.rs`: `mod llm; mod data;`.
3.  **[Task 3.3]** In `src/llm.rs`, define the structs for deserializing the LLM response.
    ```rust
    // src/llm.rs
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone)]
    pub struct LearningModule {
        pub topic: String,
        pub explanation: String,
        pub code_snippets: Vec<String>,
        pub exercises: Vec<String>,
    }
    // ... rest of the file ...
    ```
4.  **[Task 3.4]** In `src/data.rs`, implement logic to load and use the `rust_by_example_index.json`.
    ```rust
    // src/data.rs
    use serde::Deserialize;
    use anyhow::Result;
    use std::fs;

    #[derive(Debug, Deserialize, Clone)]
    pub struct Topic {
        pub topic: String,
        pub source: String,
        pub min_level: u8,
    }

    pub fn load_index() -> Result<Vec<Topic>> {
        let file_content = fs::read_to_string("data/rust_by_example_index.json")?;
        let topics: Vec<Topic> = serde_json::from_str(&file_content)?;
        Ok(topics)
    }

    pub fn select_topic_for_level(topics: &[Topic], level: u8) -> Option<&Topic> {
        use rand::seq::SliceRandom;
        let eligible_topics: Vec<&Topic> = topics.iter().filter(|t| t.min_level <= level).collect();
        eligible_topics.choose(&mut rand::thread_rng()).copied()
    }
    ```
5.  **[Task 3.5]** In `src/llm.rs`, implement the async `fetch_learning_module` function.
    ```rust
    // src/llm.rs
    // ... (imports and struct definition)
    pub async fn fetch_learning_module(api_key: &str, level: u8, topic: &str) -> anyhow::Result<LearningModule> {
        let prompt = format!(
            "You are Rust AI Mentor. Your task is to generate a learning module for a student with a skill level of {level}/10. The topic is '{topic}'.
            Provide the response in a single, minified JSON object with no newlines or formatting outside of the JSON structure.
            The JSON object must have these exact keys: 'topic' (string), 'explanation' (string, 5-10 lines, markdown is ok), 'code_snippets' (array of strings, 1-3 snippets with comments), and 'exercises' (array of strings, 1-3 small exercises).
            Base your content on official Rust documentation like 'The Rust Programming Language' book and 'Rust by Example'."
        );
        // ... (reqwest client setup, sending the prompt, and deserializing the response)
    }
    ```
6.  **[Task 3.6]** Modify the application to handle async tasks.
    *   In `src/main.rs`, load the API key using `dotenvy::dotenv().ok();`.
    *   In `src/app.rs`, add fields for async communication and data storage: `topic_index: Vec<data::Topic>`, `current_module: Option<llm::LearningModule>`, and `action_sender: mpsc::Sender<AppAction>`. Create an `AppAction` enum (e.g., `FetchModule`).
    *   Refactor the main loop in `main.rs` to use `tokio::select!` to listen for both `EventHandler` events and messages from the `AppAction` receiver. When an `AppAction` is received, spawn a `tokio` task to call the LLM and send the result back to `app` over another channel.
7.  **[Task 3.7]** In `src/app.rs`, modify the key handlers (`handle_welcome_keys`, `handle_learning_keys`) to send `AppAction::FetchModule` via the sender instead of directly changing the state.
8.  **[Task 3.8]** When the LLM result is received in the main loop, update `app.current_module` and set `app.current_state` to `AppState::Learning`.
9.  **[Task 3.9]** In `src/ui.rs`, modify `render_learning` to display the dynamic content from `app.current_module`.

---

### **Phase 4: Feature Completion & Polish**
*   **Goal:** Implement the final MVP features (**F007**), add syntax highlighting (**F005** enhancement), handle errors gracefully, and prepare for release.

1.  **[Task 4.1]** **[F007]** Ensure the "Request New Module" feature is fully functional. The `n` key handler in `app.rs` should already be sending the `AppAction::FetchModule`. Verify this flow works correctly, showing the loading screen and then new content.
2.  **[Task 4.2]** Add syntax highlighting dependencies to `Cargo.toml`:
    ```toml
    [dependencies]
    # ... (existing dependencies)
    ratatui-syntect = "0.3.0"
    syntect = { version = "5.1", features = ["default-fancy"] }
    ```
3.  **[Task 4.3]** In `src/ui.rs`, integrate `ratatui-syntect` into `render_learning`.
    *   Load `SyntaxSet` and `ThemeSet` once and store them, perhaps in a lazy_static block or in the `App` struct.
    *   When rendering code snippets, use `ratatui_syntect::SyntacticallyHighlightedText` instead of a plain `Paragraph` to get colored output.
4.  **[Task 4.4]** Implement robust error handling.
    *   In `src/app.rs`, add `last_error: Option<String>` to the `App` struct.
    *   When an async task (like the LLM fetch) fails, send the error message back to the main loop. Store it in `app.last_error` and set `current_state` to `AppState::Learning` (or a dedicated `Error` state).
    *   In `src/ui.rs`, modify `render_learning` (or create `render_error`) to display the error message prominently if `app.last_error` is `Some`.
5.  **[Task 4.5]** **[F006]** Conduct a final review of the entire codebase to ensure it remains stateless as per the MVP scope. No data should be written to disk, and each application run should be a fresh start.
6.  **[Task 4.6]** Create a comprehensive `README.md` file including:
    *   A brief description of the project.
    *   Prerequisites (`rustc`, `cargo`).
    *   How to build and run the application (`cargo run`).
    *   A list of keybindings.
7.  **[Task 4.7]** Perform final User Acceptance Testing (UAT):
    *   Compile and run the application on both Linux and macOS.
    *   Test in macos terminal.
    *   Verify all keybindings work as expected.
    *   Verify the UI is responsive and does not freeze during LLM calls.
    *   Check for any rendering glitches or text alignment issues.
// src/components/mod.rs
// This file will serve as the entry point for reusable UI components

// In a more complete implementation, we would add component modules here:
// pub mod level_selector;
// pub mod content_view;
// pub mod help_modal;

// For now, this file is a placeholder as required by the project structure
