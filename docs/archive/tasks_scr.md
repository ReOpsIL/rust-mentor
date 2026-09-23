

### **Objective:** Implement the TUI screens for the "Rust AI Mentor" application.

**Primary Tools:**
*   **Rust** language
*   **`ratatui`** crate for TUI widgets and layout.
*   **`crossterm`** as the terminal backend.

**Core Principle:** The UI rendering logic, located primarily in `src/ui.rs`, should be stateless. It will receive the current `App` state struct (from `src/app.rs`) and render the TUI frame based on that state.

---

### **Task 0: UI Rendering Foundation (`src/ui.rs`)**

**Goal:** Create the main rendering function that will dispatch to the correct view-rendering function based on the application's state.

1.  **Create the main `render` function:**
    *   Define a public function `pub fn render(frame: &mut Frame, app: &App)`.
    *   This function will be the single entry point for drawing the entire UI in each tick of the main loop.

2.  **Implement a state-based dispatch:**
    *   Inside `render`, use a `match` statement on `app.current_view` (assuming an enum `View` in `app.rs` with variants like `Welcome`, `Loading`, `Learning`).
    *   Each arm of the `match` will call a specific function to render that view (e.g., `render_welcome_view(frame, app)`).

3.  **Implement Modal Overlays:**
    *   After the main view is rendered, add `if` conditions to check if a modal should be displayed (e.g., `if app.show_help_modal`).
    *   If true, call the corresponding modal rendering function (e.g., `render_help_modal(frame)`). This ensures modals are drawn on top of the active view.

---

### **Task 1: Implement the "Welcome / Level Selection View"**

**Goal:** Create the initial screen where the user selects their skill level.

1.  **Create the function `render_welcome_view(frame: &mut Frame, app: &App)`**.

2.  **Define the main layout:**
    *   Use `Layout::default()` to create a vertical layout with three chunks:
        *   `Constraint::Length(3)` for the title bar.
        *   `Constraint::Min(0)` for the main content area.
        *   `Constraint::Length(3)` for the footer/status bar.

3.  **Render the Title Bar:**
    *   Create a `Paragraph` with the text `"Rust AI Mentor v0.1.0"`.
    *   Style the text as `Bold` and `Cyan`.
    *   Render it in the top layout chunk inside a `Block` with `Borders::BOTTOM`.

4.  **Render the Main Content Area:**
    *   Use another vertical `Layout` within the main content chunk to center the content. Use flexible `Constraint::Percentage` to create empty space above and below.
    *   Create a `Paragraph` for the welcome text (`"Welcome to the Rust AI Mentor!"`, etc.). Use `Alignment::Center`.
    *   Create a `Vec<Line>` to hold the list of selectable levels.
    *   Iterate through your defined levels (e.g., from a static array or enum). For each level:
        *   Check if it is the currently selected level by comparing with `app.selected_level`.
        *   **If selected:** Create a `Line` with a prefix (e.g., `"> "`) and style the entire line with a `Black` foreground and `Cyan` background.
        *   **If not selected:** Create a standard `Line` with just the level's text.
    *   Render the `Vec<Line>` as a `Paragraph` with `Alignment::Center`.
    *   Add a final `Paragraph` below the list for the `"[ Press Enter to Begin ]"` prompt.

5.  **Render the Footer:**
    *   Create a `Paragraph` for the keybindings. Use a `Line` containing multiple `Span`s to align text.
    *   Left-aligned span: `"(k/↑, j/↓) Change Level | (?) Help"`
    *   Right-aligned span: `"(q) Quit"`
    *   Render it in the bottom layout chunk inside a `Block` with `Borders::TOP`.

---

### **Task 2: Implement the "Main Learning View"**

**Goal:** Display the AI-generated topic, explanation, code, and exercises.

1.  **Create the function `render_learning_view(frame: &mut Frame, app: &App)`**.

2.  **Define the main layout:**
    *   Use the same three-chunk vertical layout as the Welcome view.

3.  **Render the Title Bar:**
    *   Create a `Paragraph` with text dynamically generated from the app state: `format!("Rust AI Mentor :: Level {}", app.selected_level)`.
    *   Style and render it as in the Welcome view.

4.  **Render the Main Content Area:**
    *   This area must be scrollable.
    *   Retrieve the current learning module from `app.current_module`.
    *   Construct a `Text` widget (which is a `Vec<Line>`) containing the entire module content.
    *   **Topic:** Create a `Line` for the topic header (e.g., `## TOPIC: ...`). Style it `Bold` and `Cyan`.
    *   **Explanation:** Create `Line`s for the explanation text.
    *   **Code Snippets & Exercises:** For each code block:
        *   Use the `ratatui-syntect` library to generate syntax-highlighted `Text`.
        *   Render this `Text` inside a `Paragraph` which is itself inside a `Block` to create a visual container for the code.
    *   Combine all `Line`s and code `Paragraph`s into a single scrollable `Paragraph`.
    *   Set the scroll offset using `.scroll((app.scroll_offset, 0))`.

5.  **Render the Footer:**
    *   Create a `Paragraph` with the specific keybindings for this view: `"(n) New Module | (k/↑, j/↓) Scroll | (?) Help | (q) Quit"`.
    *   Render it in the bottom layout chunk.

---

### **Task 3: Implement the "Loading View"**

**Goal:** Provide visual feedback while the app is waiting for an API call.

1.  **Create the function `render_loading_view(frame: &mut Frame, app: &App)`**.
2.  **Use the standard three-chunk layout**.
3.  **Render the Title Bar** exactly as in the "Main Learning View" to maintain context.
4.  **Render the Main Content Area:**
    *   Create a `Paragraph` with the text `"Generating your learning module... ⏳"`.
    *   Set `Alignment::Center`.
    *   Use a vertical `Layout` with flexible constraints to vertically center the `Paragraph`.
5.  **Render the Footer:**
    *   Create a simple `Paragraph` with the text `"Please wait..."` aligned to the center.

---

### **Task 4: Implement the Modals (as Overlays)**

**Goal:** Create reusable logic to render centered modals on top of the existing view.

1.  **Create a generic `render_modal` helper function:**
    *   It should take the `frame`, a desired size (`Rect`), and a `Widget` to render.
    *   Inside, it should first render `Clear` over the `Rect` to erase the background.
    *   Then, it should render the passed `Widget` (typically a `Block` containing a `Paragraph`) within that `Rect`.

2.  **Implement the "Help Modal":**
    *   **Create the function `render_help_modal(frame: &mut Frame)`**.
    *   Define the content as a `Paragraph` inside a `Block` with the title "Keybindings".
    *   The `Paragraph` text should list all keybindings as described in the wireframe.
    *   Calculate a centered `Rect` (e.g., 50% width, 50% height).
    *   Call the `render_modal` helper with this `Rect` and content.

3.  **Implement the "Quit Confirmation Modal":**
    *   **Create the function `render_quit_modal(frame: &mut Frame, app: &App)`**.
    *   Define the content `Paragraph` with the prompt "Are you sure you want to quit?".
    *   Create a `Line` for the "Yes / No" options.
    *   Use `app.quit_confirmation_selected` (or similar state) to determine which option is highlighted (e.g., render `"[ Yes ]"` with a `Reverse` style modifier).
    *   Calculate a small, centered `Rect`.
    *   Call the `render_modal` helper.