
## **Rust AI Mentor: System Architecture Overview**

### **1. Introduction**

This document outlines the high-level system architecture for the **Rust AI Mentor**, a standalone Terminal User Interface (TUI) application. The application's purpose is to accelerate the Rust learning process by providing an AI-powered, interactive learning environment directly within the developer's terminal. It delivers personalized, bite-sized lessons, code examples, and exercises based on the user's self-assessed skill level, leveraging an LLM to generate content sourced from official Rust documentation.

### **2. Architectural Goals**

The architecture is designed to meet the following key non-functional requirements, ensuring a high-quality user experience and a maintainable codebase:

*   **Responsiveness:** The TUI must remain interactive and fluid at all times. Asynchronous operations, specifically network calls to the LLM, must not block the UI thread.
*   **Maintainability & Modularity:** The codebase will be organized into logical, decoupled components (UI, core logic, external services) to simplify development, testing, and future enhancements.
*   **Portability:** The application must function consistently across primary target platforms (Linux, macOS) and a wide range of modern terminal emulators.
*   **Low Resource Usage:** As a terminal-native application, it should be lightweight in terms of CPU and memory consumption, providing a fast and efficient experience.
*   **Configuration Persistence:** The application persists user configuration settings (model selection, learning resources visibility, content customization, etc.) in a configuration file (`~/rust-mentor.conf`), allowing users to maintain their preferences between sessions.
*   **Session-Based Learning:** While the application does not persist user progress or learning history, it maintains state within a session, allowing users to navigate between different features and return to their current learning module.

### **3. Major Application Components**

The application is structured into five primary internal components and one external interface, all orchestrated by a central event loop.

*   **UI / Presentation Layer**
    *   **Role:** This layer is responsible for everything the user sees and interacts with. It renders all TUI views, widgets, and modals as defined in the UX specification. It also captures raw keyboard input events from the terminal.
    *   **Responsibilities:**
        *   Drawing the UI frame-by-frame based on the current application state.
        *   Handling low-level terminal operations (entering/exiting raw mode, clearing screen).
        *   Translating raw keyboard events into application-specific actions or commands.
    *   **Technologies:** `ratatui` (for widgets and layout), `crossterm` (for the terminal backend and event handling).

*   **Application Logic / Core**
    *   **Role:** This is the heart of the application, containing the central state machine and business logic. It acts as the coordinator between the UI and other components.
    *   **Responsibilities:**
        *   Managing the application's global state (e.g., current view, selected skill level, selected content source, loaded content).
        *   Processing user actions received from the UI layer and triggering appropriate state transitions.
        *   Orchestrating asynchronous tasks, such as fetching data from the LLM client.
        *   Handling application-level errors and updating the state accordingly.
        *   Managing different application states:
            * Welcome - Initial screen for level selection
            * IndexSelection - Screen for selecting content source (Rust Library, Rust By Example, The Rust Programming Language, Random)
            * Learning - Main screen displaying the learning content
            * Loading - Screen shown while waiting for LLM response
            * LevelTooLowPopup - Error screen shown when user level is too low for selected content
            * Settings - Screen for customizing learning resources, content, and question generator settings
            * QuestionGeneration - State while generating questions based on the current learning module
            * QuestionAnswering - Screen for answering generated questions
            * ApplicationGeneration - State while generating a sample application based on answered questions
            * ApplicationDisplay - Screen for viewing and creating a Cargo project from the generated application
    *   **Technologies:** Custom Rust structs for state management, `tokio` for the async runtime and `mpsc` channels for message passing between async tasks and the main event loop.

*   **Data Access Layer**
    *   **Role:** This layer is responsible for accessing data stored on the local filesystem. For the MVP, its role is limited to read-only access of pre-packaged application data.
    *   **Responsibilities:**
        *   Loading and deserializing multiple data sources at startup:
            * `rust_library_index.json` - Rust standard and community libraries
            * `rust_by_example_full.json` - Rust By Example content
            * `the_rust_programming_language.json` - The Rust Programming Language book
        *   Providing an interface for the Application Core to select a relevant topic from these indices based on the user's skill level and chosen content source.
    *   **Technologies:** `serde` and `serde_json` for deserializing the JSON files into Rust structs.

*   **External Service Clients**
    *   **Role:** This component encapsulates all communication with external network APIs. It is responsible for making outbound requests and parsing responses.
    *   **Responsibilities:**
        *   Constructing structured prompts for the LLM based on input from the Application Core.
        *   Handling HTTP requests to the OpenRouter API, including API key authentication.
        *   Parsing the JSON response from the LLM into a structured format that the Application Core can use.
        *   Supporting different LLM models through the OpenRouter API.
    *   **Technologies:** `reqwest` for making asynchronous HTTP requests.

*   **Configuration Management**
    *   **Role:** This component manages user configuration settings and preferences.
    *   **Responsibilities:**
        *   Loading and saving configuration settings to/from the filesystem.
        *   Providing default configuration values for first-time users.
        *   Managing user preferences for learning resources, content customization, and question generator settings.
        *   Supporting over 40 specialized learning goals for personalized content generation.
    *   **Technologies:** `toml` and `toml_edit` for configuration file parsing, `directories` for finding user directories.

*   **Question & Application Generator**
    *   **Role:** This component generates quiz questions and sample applications based on the learning content.
    *   **Responsibilities:**
        *   Creating binary (Yes/No) and multiple-choice questions based on the current learning module.
        *   Generating sample Rust applications based on answered questions.
        *   Creating Cargo projects for learning modules and generated applications.
        *   Managing the question answering flow and application display.
    *   **Technologies:** Uses the LLM client for question and application generation, `tokio` for asynchronous processing.

### **4. Data Flow Explanation**

Below are the data flows for key user interactions in the application.

#### **4.1 Requesting a New Learning Module**

1.  **User Input:** The user, in the **Main Learning View**, presses the `n` key.
2.  **UI Layer:** `crossterm` captures the key press event. The main event loop in `tui.rs` receives this event and passes it to the **Application Core**.
3.  **Application Core:** The `App` state machine handles the event. It transitions its internal state to a `Loading` view state, which causes the UI to re-render with a loading indicator on the next tick.
4.  **Async Task Spawn:** The Core spawns an async `tokio` task to fetch the new content.
5.  **Data Access:** Within the task, a function calls the **Data Access Layer** to select a suitable topic from the appropriate index (based on the selected content source) and the user's skill level.
6.  **External API Call:** The task then calls the **LLM Client**, passing the selected topic and level. The client constructs a prompt (incorporating content customization settings) and uses `reqwest` to send an async HTTPS request to the OpenRouter API.
7.  **Response & Update:** The `tokio` task awaits the LLM's response. Upon receiving a successful response, it parses the content and sends the complete, structured learning module back to the **Application Core** via a `tokio::mpsc` channel. The Core updates its state with the new content and changes the view state from `Loading` to `Learning`.
8.  **UI Update:** On the next render tick, the **UI / Presentation Layer** reads the updated state from the Application Core, sees the new content and the `Learning` view state, and draws the refreshed Main Learning View to the terminal.

#### **4.2 Generating and Answering Questions**

1.  **User Input:** The user, in the **Main Learning View**, presses the `w` key to generate questions.
2.  **Application Core:** The state machine transitions to the `QuestionGeneration` state and spawns an async task to generate questions based on the current learning module.
3.  **Question Generation:** The **Question Generator** component uses the LLM to create a set of questions (binary or multiple-choice) related to the current topic.
4.  **State Update:** When the questions are ready, the state transitions to `QuestionAnswering`, and the UI displays the first question.
5.  **User Interaction:** The user navigates between questions and selects answers. The Application Core tracks the user's progress.
6.  **Application Generation:** After answering all questions, the user can press Enter to generate a sample application based on their answers.
7.  **Application Display:** The generated application is displayed, and the user can create a Cargo project from it by pressing Enter again.

#### **4.3 Configuration Management**

1.  **User Input:** The user presses the `s` key to access settings.
2.  **Application Core:** The state machine transitions to the `Settings` state.
3.  **UI Update:** The Settings screen is displayed, showing the current configuration options.
4.  **User Interaction:** The user navigates between settings sections (Learning Resources, Content Customization, Learning Goals, Question Generator) and modifies options.
5.  **Configuration Persistence:** When a setting is changed, the **Configuration Management** component saves the updated configuration to the `~/rust-mentor.conf` file.
6.  **Application Behavior:** The updated settings affect subsequent content generation, question generation, and resource display.

### **5. Technology Choices with Reasoning**

*   **Language (Rust):** Chosen for its performance, memory safety, and strong ecosystem for building robust CLI and TUI applications. This aligns with the goals of low resource usage and maintainability.
*   **TUI Framework (`ratatui` & `crossterm`):** This combination is the community standard, offering a stable, well-documented, and portable foundation for building complex TUIs, directly supporting the **Portability** and **Maintainability** goals.
*   **Async Runtime (`tokio`):** Essential for achieving the **Responsiveness** goal. It allows network-bound operations (LLM calls) to run in the background without freezing the user interface.
*   **State Management (Custom Struct & `mpsc` channels):** A simple, self-contained state machine is sufficient for the MVP's scope and supports the **Maintainability** goal by keeping state logic explicit and centralized.
*   **Data Serialization (`serde_json`):** The de-facto standard for working with JSON in Rust, providing a reliable and performant way to implement the **Data Access Layer**.

### **6. Component Diagram (Mermaid.js Syntax)**

```mermaid
graph TD
    User([User]) -- Keyboard Input --> A[UI / Presentation Layer<br/>(ratatui + crossterm)]

    subgraph Rust AI Mentor Application
        A -- User Actions (e.g., 'n' key) --> B[Application Logic / Core<br/>(State Machine, Event Loop)]
        B -- Renders State --> A

        B -- Triggers LLM Fetch --> D[External Service Client<br/>(LLM Client / reqwest)]
        D -- Fetches Content --> B

        B -- Requests Topic --> C[Data Access Layer<br/>(serde_json)]
        C -- Reads From --> E[(Local Filesystem<br/>data/rust_library_index.json<br/>data/rust_by_example_full.json<br/>data/the_rust_programming_language.json)]

        B -- Manages Settings --> G[Configuration Management<br/>(toml, directories)]
        G -- Reads/Writes --> H[(User Configuration<br/>~/rust-mentor.conf)]

        B -- Generates Questions/Apps --> I[Question & Application Generator]
        I -- Uses --> D
    end

    D -- HTTPS API Call --> F([External OpenRouter API])

    style User fill:#ddd,stroke:#333,stroke-width:2px
    style F fill:#f9f,stroke:#333,stroke-width:2px
```
