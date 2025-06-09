

# Rust AI Mentor: TUI Design Specification

## 1. Introduction

This document outlines the design specification for the Terminal User Interface (TUI) of the **Rust AI Mentor** application. The application's purpose is to provide an AI-powered, terminal-based learning environment for Rust developers.

The target audience consists of developers, students, and hobbyists who are comfortable working within a terminal environment.These users value efficiency, focus, and keyboard-driven workflows. The TUI is designed to provide a distraction-free, interactive, and personalized learning experience, directly addressing the common challenges of learning Rust, such as information overload and context switching.

## 2. TUI/UX Considerations

### **Overall Look & Feel**

The TUI will adopt a **modern, utility-focused aesthetic**. It will utilize box-drawing characters to create clear panes and modals, with a purposeful use of color to guide the user's attention and improve readability. The design will be clean and minimalist, avoiding unnecessary clutter to support the "Distraction-Free TUI Experience" value proposition.

### **Key Usability Goals**

*   **Keyboard-First Efficiency:** All interactions will be achievable via the keyboard. Mouse support is not an MVP requirement.
*   **Intuitive Keybindings:** Keybindings will be simple and contextually displayed (e.g., in a footer/status bar) to minimize cognitive load.
*   **Clear Information Hierarchy:** Strong visual hierarchy using titles, text styling (bold, dim), and layout will make content easy to scan and digest.
*   **Low Latency Perception:** The UI must remain responsive. Asynchronous operations (like fetching content from the LLM) will display a clear loading state to manage user expectations.
*   **Focus on Readability:** Ample padding, appropriate line spacing, and syntax highlighting for code are crucial for comfortable reading within the terminal.

### **Terminal & Platform Considerations**

*   **Cross-Platform Compatibility:** The application must render correctly on modern terminal emulators across Linux, macOS.
*   **Responsiveness:** The layout will be designed to adapt gracefully to different terminal window sizes. At very small sizes, readability is prioritized over layout integrity.
*   **Color Support:** The design will primarily use the ANSI 16-color palette for maximum compatibility but will gracefully degrade on terminals with limited color support (e.g., monochrome).
*   **SSH Compatibility:** The TUI should function correctly over a standard SSH session.

## 3. User Personas

### **Persona 1: Alex, the Aspiring Rustacean**

*   **Role:** Computer Science Student, 20
*   **Goals:** Learn Rust fundamentals for a university project. Wants a guided path that builds confidence and provides immediate practice.
*   **Frustrations:** Feels overwhelmed by the size of "The Rust Programming Language" book. Finds it hard to connect concepts to small, practical code examples. Constantly switches between a web browser, his code editor, and the terminal, breaking his focus.
*   **Terminal/CLI Proficiency:** Intermediate. Comfortable with basic shell commands and navigating files.
*   **Scenario:** Alex launches `rust-ai-mentor` to prepare for his next lecture. He selects skill level "2". The app presents a module on "Ownership." He reads the concise explanation, studies the code snippets, and then tries to solve the small exercises in his editor. Feeling more confident, he requests another module and gets one on "Structs."

### **Persona 2: Priya, the Transitioning Developer**

*   **Role:** Backend Developer (Python/Go), 32
*   **Goals:** Quickly get up to speed with Rust for a new role. Needs to efficiently map her existing programming knowledge to Rust's unique paradigms (borrowing, lifetimes, error handling).
*   **Frustrations:** Reading introductory material that re-explains basic programming concepts. Struggles to find targeted information on intermediate topics without wading through beginner tutorials.
*   **Terminal/CLI Proficiency:** High. Lives in the terminal, uses Vim, and is proficient with `git`, `docker`, and other CLI tools.
*   **Scenario:** Priya wants to solidify her understanding of Rust's `Result` and `Option` enums. She starts the app, selects skill level "5", and gets a module on `match` expressions. It's relevant, but not what she's looking for. She uses the "Request New Module" feature. The next module is on "Error Handling with `Result`," which is perfect. She studies the examples, comparing them to try-except blocks in Python, and feels ready to refactor some code.

## 4. User Journeys

### **Journey 1: A Complete Learning Session (Alex)**

1.  **Launch:** Alex opens his terminal and runs `rust-ai-mentor`.
2.  **Level Select:** The **Welcome / Level Selection View** appears. The selector is focused on "5". He uses the `Down Arrow` key to move to "2" and presses `Enter`.
3.  **Index Select:** The **Index Selection View** appears. He uses the `Down Arrow` key to select "Rust By Example" and presses `Enter`.
4.  **Loading:** A **Loading View** appears briefly with a message: "Generating your first learning module..."
5.  **Learn:** The **Main Learning View** is displayed with a module on "Ownership." Alex reads the content, scrolling down with `j` or `Down Arrow` if necessary.
6.  **Request New:** After finishing, he presses `n` to request a new module.
7.  **Loading:** The **Loading View** appears again.
8.  **Learn Again:** The **Main Learning View** updates with a new module on "Structs."
9.  **Help:** He forgets how to quit and presses `?`. The **Help Modal** appears, showing the keybindings. He sees `q: Quit`. He presses `Esc` to close the modal.
10. **Quit:** He presses `q`. A **Quit Confirmation Modal** appears. He presses `Enter` to confirm.
11. **Exit:** The application closes and he is returned to his shell prompt.

### **Journey 2: Quick Topic Refresher (Priya)**

1.  **Launch:** Priya runs `rust-ai-mentor`.
2.  **Level Select:** The **Welcome / Level Selection View** appears. She uses the `Up Arrow` to select "6" and presses `Enter`.
3.  **Index Select:** The **Index Selection View** appears. She selects "The Rust Programming Language" and presses `Enter`.
4.  **Loading:** The **Loading View** appears briefly.
5.  **Learn:** The app generates and displays a module on "Traits." She reviews the code snippets, which use syntax highlighting.
6.  **Interact:** She uses her mouse to select and copy a code snippet from the terminal to her Neovim session in another pane to experiment with it.
7.  **Quit:** Satisfied, she presses `q`, then `Enter` to quit the application.

## 5. Key Views & Navigation Model

### **List of Views**

1.  **Welcome / Level Selection View:** The initial screen. Prompts the user to select their skill level.
2.  **Index Selection View:** After selecting a skill level, this screen prompts the user to choose a content source (Rust Library, Rust By Example, The Rust Programming Language, or Random).
3.  **Loading View:** A simple, temporary view displayed during LLM API calls.
4.  **Main Learning View:** The core screen of the application. Displays the topic, explanation, code snippets, and exercises.
5.  **Help Modal:** An overlay that displays a list of available keybindings.
6.  **Quit Confirmation Modal:** A simple modal to prevent accidental quitting.

### **Primary Navigation Model**

The navigation is **modal and context-driven**, designed for simplicity and discoverability. There are no complex modes like in Vim.

*   **Global Keys:**
    *   `q`: Quit (with confirmation) or go back/close modal.
    *   `?`: Show/hide the Help Modal.
*   **View-Specific Keys:**
    *   **Level Selection:** `Up/Down` or `k/j` to change level, `Enter` to confirm.
    *   **Index Selection:** `Up/Down` or `k/j` to change content source, `Enter` to confirm.
    *   **Main Learning View:** `n` to request a new module. `Up/Down` or `k/j` to scroll content if it overflows the viewport.
*   **Interaction Flow:**
    `Launch` -> `Welcome View` -> `(Select Level)` -> `Index Selection View` -> `(Select Content Source)` -> `Loading View` -> `Main Learning View`
    From `Main Learning View`:
    *   Pressing `n` -> `Loading View` -> `Main Learning View (new content)`
    *   Pressing `?` -> `Help Modal (overlay)` -> `(Press Esc/q)` -> `Main Learning View`
    *   Pressing `q` -> `Quit Modal (overlay)` -> `(Press Enter)` -> `Exit App`

## 6. TUI Design Principles & Style Guide

### **Core Principles**

*   **Keyboard is King:** Every action is mapped to a key.
*   **Provide Contextual Help:** Key actions are always visible in the footer/status bar.
*   **Consistent Layout:** Views share a common structure (e.g., title bar, content area, status bar) for predictability.
*   **Use Color Purposefully:** Color is used to draw attention, denote state, and improve code readability, not for decoration.
*   **Clarity Over Density:** Prioritize readability with whitespace and clear separation of elements.

### **Styling & Theming (ANSI 16/256 Palette)**

*   **Color Palette:**
    *   **Borders/Frames:** `Dim White` or `Gray`
    *   **Titles/Headers:** `Bold Cyan`
    *   **Primary Text:** `White`
    *   **Labels/Prompts:** `Yellow`
    *   **Selected/Focused Item:** `Black` text on `Cyan` background.
    *   **Success/Confirmation:** `Green`
    *   **Warning/Confirmation Prompt:** `Yellow`
    *   **Error:** `Red`
    *   **Syntax Highlighting:** A standard, high-contrast theme (e.g., similar to Gruvbox or Monokai) will be used via a Rust library.
*   **Text Styling:**
    *   **Titles:** `Bold`
    *   **Labels:** `Normal` or `Dim`
    *   **Emphasized Text:** _Italics_ (where supported)

## 7. ASCII Wireframes/Layouts

### **Wireframe 1: Welcome / Level Selection View**

```
+--------------------------------------------------------------------------+
| Rust AI Mentor v0.1.0                                                    |
+--------------------------------------------------------------------------+
|                                                                          |
|                           Welcome to Rust AI Mentor!                     |
|                                                                          |
|              Please select your current Rust skill level:                |
|                                                                          |
|                 ┌──────────────────────────────────────┐                 |
|                 │        1 .................... 10       │                 |
|                 │        ########▓───────────────────    │                 |
|                 │              ▲ (Level 4)             │                 |
|                 └──────────────────────────────────────┘                 |
|                     (Use ←/→ or h/l to adjust)                         |
|                                                                          |
|                                                                          |
|                          [ Press Enter to Begin ]                        |
|                                                                          |
+--------------------------------------------------------------------------+
| (?) Help                                                             (q) Quit |
+--------------------------------------------------------------------------+
```
*(Note: An alternative to the slider is a simple list `[>] Level 4` that the user navigates with up/down arrows.)*

### **Wireframe 2: Main Learning View**

```
+--------------------------------------------------------------------------+
| Rust AI Mentor :: Level 4                                                |
+--------------------------------------------------------------------------+
|                                                                          |
| ## TOPIC: Understanding Ownership                                        |
|                                                                          |
| Explanation:                                                             |
|   In Rust, every value has a variable that’s called its owner. There can |
|   only be one owner at a time. When the owner goes out of scope, the     |
|   value will be dropped. This rule is checked at compile time.           |
|                                                                          |
| ---                                                                      |
|                                                                          |
| Code Snippets:                                                           |
|                                                                          |
|   // 1. Ownership and Scope                                              |
|   {                                                                      |
|       let s = String::from("hello"); // s is valid from this point forward|
|       // do stuff with s                                                 |
|   } // this scope is now over, and s is no longer valid                   |
|                                                                          |
| ---                                                                      |
|                                                                          |
| Exercises:                                                               |
|                                                                          |
|   1. What is the error in the code below? Explain why it happens.        |
|      let s1 = String::from("hello");                                      |
|      let s2 = s1;                                                        |
|      println!("{}, world!", s1);                                         |
|                                                                          |
+--------------------------------------------------------------------------+
| (n) New Module | (k/↑, j/↓) Scroll | (?) Help | (q) Quit                 |
+--------------------------------------------------------------------------+
```

### **Wireframe 3: Help Modal (Overlay)**

```
+--------------------------------------------------------------------------+
| Rust AI Mentor :: Level 4                                                |
+------------------------------------------------------------------+-------+
|                                                                  |       |
| ## TOPIC: Understanding Ownership                              |       |
|                                                                  |       |
| Explanation:                                                     |       |
|   In Rust, every value has a variab┌───────────────────────────┐ |       |
|   only be one owner at a time. Whe │         Keybindings         │ |       |
|   value will be dropped. This rule ├───────────────────────────┤ |       |
|                                    │ n: Request New Module     │ |       |
| ---                                │ q: Quit Application       │ |       |
|                                    │ ?: Toggle this Help menu  │ |       |
| Code Snippets:                     │ k,↑: Scroll Up            │ |       |
|                                    │ j,↓: Scroll Down          │ |       |
|   // 1. Ownership and Scope        │ Esc: Close this menu      │ |       |
|   {                                │                           │ |       |
|       let s = String::from("hello" └───────────────────────────┘ |       |
|       // do stuff with s                                          |       |
|   } // this scope is now over, and s is no longer valid           |       |
|                                                                  |       |
+------------------------------------------------------------------+-------+
| (n) New Module | (k/↑, j/↓) Scroll | (?) Help | (q) Quit                 |
+--------------------------------------------------------------------------+
```
