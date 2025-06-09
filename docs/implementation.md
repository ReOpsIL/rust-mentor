
## **Rust AI Mentor: Technical Implementation Plan**

### **Overview**

This document outlines the technical strategy for building the Minimum Viable Product (MVP) of the **Rust AI Mentor** TUI application. It is based on the provided App Idea, Feature List, and TUI Design Specification. The plan prioritizes a robust foundation, modular design, and a clear, phased development approach to deliver the core value proposition efficiently.

### **1. Technology Stack & Crate Suggestions**

The selection of technologies is focused on community support, performance, cross-platform compatibility, and developer ergonomics within the Rust ecosystem.

*   **TUI Framework:**
    *   **Crate:** `ratatui`
    *   **Reasoning:** As a community-driven fork of the venerable `tui-rs`, `ratatui` offers a stable, well-documented, and actively maintained foundation. Its immediate-mode rendering paradigm is flexible, and its rich ecosystem of widgets and examples is ideal for implementing the specified layouts and components. It provides full control over the terminal buffer, which is essential for our custom views.

*   **Terminal Backend:**
    *   **Crate:** `crossterm`
    *   **Reasoning:** `crossterm` is the de-facto standard for terminal manipulation in Rust. It provides robust, cross-platform support (Linux, macOS, Windows) for raw mode, event handling (keyboard, mouse), and styling. Its integration with `ratatui` is seamless and well-tested.

*   **Core Application Logic & State Management:**
    *   **Crates:** `tokio` (for async runtime), `thiserror` / `anyhow` (for error handling).
    *   **Reasoning:** The LLM integration (F003) is an I/O-bound network operation, making an asynchronous approach essential to prevent the UI from freezing. `tokio` is the industry-standard async runtime in Rust. We will use a main event loop that polls for both terminal events and results from async tasks (like LLM responses), likely managed through `tokio::mpsc` channels. A simple state machine implemented in an `App` struct will manage the application's current view (e.g., `Welcome`, `Learning`, `Help`). `anyhow` will be used for application-level error handling, and `thiserror` for creating specific, typed errors where needed (e.g., for LLM client interactions).

*   **Data & Configuration:**
    *   **Crates:** `serde` & `serde_json` (for topic indices), native rust (for API keys).
    *   **Reasoning:** Per feature F004, we will use pre-created JSON indices of topics from multiple sources: Rust libraries, "Rust by Example", and "The Rust Programming Language" book. `serde` and `serde_json` are the perfect tools for deserializing these files into Rust structs at startup. For managing the OpenRouter API key, we use environment variables with native rust.

*   **Key Supporting Crates:**
    *   **HTTP Client:** `reqwest` - A high-level, ergonomic async HTTP client built on `tokio`, perfect for interacting with the OpenRouter API.
    *   **Syntax Highlighting:** `syntect` paired with `ratatui-syntect` - To fulfill the requirement in F005 for code highlighting, this combination provides a powerful way to parse and render code with syntax themes directly into a `ratatui` buffer.
    *   **Logging:** `tracing` - Provides structured, context-aware logging that works exceptionally well with `tokio`'s asynchronous tasks. This will be invaluable for debugging API calls and application state changes.

### **2. Development Phases and Milestones (MVP)**

The MVP will be developed in four distinct phases, building from the foundation upwards.

---

#### **Phase 1: Project Scaffolding & TUI Foundation**
*   **Goal:** Establish a runnable application shell with a working event loop.
*   **Features/Tasks:**
    *   Initialize the Rust project and Git repository.
    *   Set up `ratatui`, `crossterm`, `tokio`, and `tracing`.
    *   Implement the main application loop to handle terminal initialization/restoration.
    *   Create the core event handling logic to listen for keyboard inputs.
    *   **F001 (Partial):** Implement basic TUI rendering with a placeholder view.
*   **Milestone:** **The application launches into a blank TUI screen, remains responsive, and cleanly exits upon pressing the 'q' key. Basic logging is functional.**

---

#### **Phase 2: Static UI Implementation & Navigation**
*   **Goal:** Build all required UI views and modals with static/mock data.
*   **Features/Tasks:**
    *   Implement the `App` state machine to manage the current view (`State::Welcome`, `State::Learning`, etc.).
    *   **F001/F005:** Create the rendering logic for the primary views defined in the UX spec:
        *   Welcome / Level Selection View
        *   Main Learning View (using hardcoded mock content)
        *   Help Modal
        *   Quit Confirmation Modal
    *   **F002 (UI only):** Implement the level selector widget.
    *   Implement view navigation based on keybindings (`Enter` from Welcome, `?` for Help, `q` to trigger Quit modal).
*   **Milestone:** **All views from the wireframes are rendered correctly with mock data. The user can navigate between the Welcome screen, Main Learning view, and Help modal using the specified keybindings.**

---

#### **Phase 3: LLM Integration & Dynamic Content**
*   **Goal:** Replace mock data with live, AI-generated content.
*   **Features/Tasks:**
    *   **F003:** Implement the OpenRouter API client using `reqwest`. Handle API key configuration via environemnt variables.
    *   Create robust prompt engineering logic to generate topics, explanations, code, and exercises based on the user's level.
    *   Load and utilize the "Rust by Example" JSON index to select a topic for the prompt (per F004).
    *   **F004:** Integrate the LLM client with the application state. When a level is selected, trigger an async task to call the LLM.
    *   Implement the `LoadingView` to display while waiting for the API response.
    *   Manage the response via `tokio::mpsc` channels, updating the `App` state upon completion or error.
*   **Milestone:** **Upon selecting a skill level, the app displays a loading indicator, calls the OpenRouter API, and successfully populates the Main Learning View with dynamically generated content.**

---

#### **Phase 4: Feature Completion & Polish**
*   **Goal:** Finalize all MVP features and refine the user experience.
*   **Features/Tasks:**
    *   **F007:** Implement the "Request New Module" feature, which re-triggers the LLM generation task from Phase 3.
    *   **F005 (Enhancement):** Integrate `ratatui-syntect` to add syntax highlighting to all code snippets.
    *   Implement graceful error handling (e.g., display an error message in the TUI if an API call fails).
    *   Conduct thorough manual testing across target platforms (Linux, macOS) and terminals.
    *   Code cleanup, documentation, and final review.
*   **Milestone:** **All MVP features (F001-F007) are fully functional. The application is stable, provides a polished user experience, and is ready for initial user testing.**

---

#### **Phase 5: Advanced Features & Customization**
*   **Goal:** Implement advanced features that enhance the personalization and customization of the learning experience.
*   **Features/Tasks:**
    *   **F008:** Implement Learning Resources Customization
        *   Add configuration options for toggling visibility of different learning resources
        *   Update the UI to display or hide resources based on user preferences
    *   **F009:** Implement Content Customization
        *   Add configuration options for code complexity, explanation verbosity, and focus area
        *   Update the LLM prompt generation to incorporate these customization options
    *   **F010:** Implement Settings Management
        *   Create a dedicated Settings View in the TUI
        *   Implement navigation between settings sections
        *   Add UI components for toggling and cycling through options
        *   Ensure settings are persisted between application sessions
    *   Enhance the Help Modal to include information about the new settings features
    *   Update documentation to reflect the new features
*   **Milestone:** **Advanced features (F008-F010) are fully functional. Users can customize their learning experience through a dedicated settings interface. Settings are persisted between sessions.**

### **3. Responsibility Mapping (Conceptual)**

For a project of this scope, a small, focused team (or a single developer wearing multiple hats) is ideal.

*   **Rust Developer (Core Logic & TUI):**
    *   Primary responsibility for all phases of development.
    *   Implements the TUI layout, event loop, state management, and LLM integration.
    *   Writes unit and component tests.
    *   Manages the crate dependencies and project structure.

*   **UI/UX Designer (Consultant):**
    *   The `ux_specification.md` serves as the primary design artifact.
    *   Provides feedback on TUI readability, information hierarchy, and keyboard flow during development.
    *   Helps refine the wording of prompts and labels for clarity.

*   **QA Tester / Product Owner:**
    *   Performs User Acceptance Testing (UAT) on different platforms and terminal emulators.
    *   Verifies that the implemented features match the specification and meet user needs.
    *   Manages the backlog and prioritizes features for post-MVP releases.

### **4. Rust Project Structure / Module Breakdown**

A modular structure will be used to ensure separation of concerns. The Git strategy will be **GitHub Flow**, where `main` is always deployable and new work is done on feature branches.

```
rust-ai-mentor/
├── .cargo/
│   └── config.toml
├── .env.example        # Template for API keys
├── data/
│   ├── rust_library_index.json      # Rust libraries topic index
│   ├── rust_by_example_full.json    # Rust By Example content
│   └── the_rust_programming_language.json # The Rust Programming Language book content
├── src/
│   ├── bin/            # Binary executables
│   ├── components/     # Reusable UI widgets (e.g., level_selector.rs)
│   │   └── mod.rs
│   ├── app.rs          # Core application state machine and logic
│   ├── cargo_project.rs # Cargo project management
│   ├── config.rs       # Configuration management
│   ├── data.rs         # Data loading and management
│   ├── event.rs        # Event handling (terminal events, ticks)
│   ├── llm.rs          # LLM client, prompt generation, API interaction
│   ├── prompt_response.rs # Structured response parsing
│   ├── tui.rs          # Terminal initialization, restoration, and main loop
│   ├── ui.rs           # TUI rendering logic (maps state to ratatui widgets)
│   └── main.rs         # Entry point: sets up logging, app, and starts TUI
└── Cargo.toml
```

### **5. Testing Strategies**

A multi-layered testing approach will ensure application quality and robustness.

*   **Unit Testing (`#[test]`):**
    *   **Focus:** Pure, state-independent logic.
    *   **Examples:** Functions in `llm.rs` for prompt generation; state transition logic in `app.rs` (e.g., `app.quit()` should correctly change the `is_running` flag). These tests will be fast and run in standard `cargo test`.

*   **Component/View Testing:**
    *   **Focus:** Testing individual `ratatui` components or entire views in isolation.
    *   **Methodology:** We will use `ratatui::backend::TestBackend` to render a component to an in-memory buffer. The test will then assert that the buffer's content matches a predefined string or a snapshot file. This is perfect for verifying the layouts from the wireframes without needing a real terminal.
    *   **Example:** A test for the `HelpModal` would render it to a `TestBackend` and assert that the resulting buffer contains the correct keybinding text.

*   **Integration Testing:**
    *   **Focus:** Verifying the interaction between different modules, especially the main event loop.
    *   **Methodology:** These tests will simulate a sequence of events (e.g., key presses) being sent to the application and assert the final state of the `App` struct. This validates that event handling, state updates, and business logic work together as expected.

*   **User Acceptance Testing (UAT):**
    *   **Focus:** Manual testing of the compiled binary to catch platform-specific issues.
    *   **Checklist:**
        *   Test on target OSes: Linux, macOS.
        *   Test on macos terminal.
        *   Verify correct rendering of colors and box-drawing characters.
        *   Test UI responsiveness during window resizing.
