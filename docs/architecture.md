
## **Rust AI Mentor: System Architecture Overview**

### **1. Introduction**

This document outlines the high-level system architecture for the **Rust AI Mentor**, a standalone Terminal User Interface (TUI) application. The application's purpose is to accelerate the Rust learning process by providing an AI-powered, interactive learning environment directly within the developer's terminal. It delivers personalized, bite-sized lessons, code examples, and exercises based on the user's self-assessed skill level, leveraging an LLM to generate content sourced from official Rust documentation.

### **2. Architectural Goals**

The architecture is designed to meet the following key non-functional requirements, ensuring a high-quality user experience and a maintainable codebase:

*   **Responsiveness:** The TUI must remain interactive and fluid at all times. Asynchronous operations, specifically network calls to the LLM, must not block the UI thread.
*   **Maintainability & Modularity:** The codebase will be organized into logical, decoupled components (UI, core logic, external services) to simplify development, testing, and future enhancements.
*   **Portability:** The application must function consistently across primary target platforms (Linux, macOS) and a wide range of modern terminal emulators.
*   **Low Resource Usage:** As a terminal-native application, it should be lightweight in terms of CPU and memory consumption, providing a fast and efficient experience.
*   **Stateless Operation (MVP):** For the initial release, the application will be stateless, meaning it does not persist user progress or session history. This simplifies the architecture and focuses development on the core learning loop.

### **3. Major Application Components**

The application is structured into three primary internal components and one external interface, all orchestrated by a central event loop.

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
        *   Managing the application's global state (e.g., current view, selected skill level, loaded content).
        *   Processing user actions received from the UI layer and triggering appropriate state transitions.
        *   Orchestrating asynchronous tasks, such as fetching data from the LLM client.
        *   Handling application-level errors and updating the state accordingly.
    *   **Technologies:** Custom Rust structs for state management, `tokio` for the async runtime and `mpsc` channels for message passing between async tasks and the main event loop.

*   **Data Access Layer**
    *   **Role:** This layer is responsible for accessing data stored on the local filesystem. For the MVP, its role is limited to read-only access of pre-packaged application data.
    *   **Responsibilities:**
        *   Loading and deserializing the `rust_by_example_index.json` file at startup.
        *   Providing an interface for the Application Core to select a relevant topic from this index based on the user's skill level.
    *   **Technologies:** `serde` and `serde_json` for deserializing the JSON file into Rust structs.

*   **External Service Clients**
    *   **Role:** This component encapsulates all communication with external network APIs. It is responsible for making outbound requests and parsing responses.
    *   **Responsibilities:**
        *   Constructing structured prompts for the LLM based on input from the Application Core.
        *   Handling HTTP requests to the OpenRouter API, including API key authentication.
        *   Parsing the JSON response from the LLM into a structured format that the Application Core can use.
    *   **Technologies:** `reqwest` for making asynchronous HTTP requests.

### **4. Data Flow Explanation**

Below is the data flow for the key user interaction of requesting a new learning module.

1.  **User Input:** The user, in the **Main Learning View**, presses the `n` key.
2.  **UI Layer:** `crossterm` captures the key press event. The main event loop in `tui.rs` receives this event and passes it to the **Application Core**.
3.  **Application Core:** The `App` state machine handles the event. It transitions its internal state to a `Loading` view state, which causes the UI to re-render with a loading indicator on the next tick.
4.  **Async Task Spawn:** The Core spawns an async `tokio` task to fetch the new content.
5.  **Data Access:** Within the task, a function calls the **Data Access Layer** to select a suitable topic from the in-memory `rust_by_example_index` based on the currently stored user level.
6.  **External API Call:** The task then calls the **LLM Client**, passing the selected topic and level. The client constructs a prompt and uses `reqwest` to send an async HTTPS request to the OpenRouter API.
7.  **Response & Update:** The `tokio` task awaits the LLM's response. Upon receiving a successful response, it parses the content and sends the complete, structured learning module back to the **Application Core** via a `tokio::mpsc` channel. The Core updates its state with the new content and changes the view state from `Loading` to `Learning`.
8.  **UI Update:** On the next render tick, the **UI / Presentation Layer** reads the updated state from the Application Core, sees the new content and the `Learning` view state, and draws the refreshed Main Learning View to the terminal.

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
        C -- Reads From --> E[(Local Filesystem<br/>data/rust_by_example_index.json)]
    end

    D -- HTTPS API Call --> F([External OpenRouter API])

    style User fill:#ddd,stroke:#333,stroke-width:2px
    style F fill:#f9f,stroke:#333,stroke-width:2px
```