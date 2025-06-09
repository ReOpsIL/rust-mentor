Based on the `ux_specification.md` and the other provided documents, here are the designs for all the TUI screens and modals, presented in Markdown format.

This format uses Markdown's features like headers, code blocks, bold text, and blockquotes to simulate the intended layout, styling, and information hierarchy described in the TUI design specification.

---

## **Rust AI Mentor: TUI Screen Designs**

This document contains the Markdown representation of all user-facing views and modals for the Rust AI Mentor application.

### **1. Welcome / Level Selection View**

This is the initial screen the user sees upon launching the application. It prompts for a skill level to begin the session. The list-based selector is used as a more practical alternative to the ASCII slider.

```markdown
+--------------------------------------------------------------------------+
| **Rust AI Mentor v0.1.0**                                                |
+--------------------------------------------------------------------------+
|                                                                          |
|                   **Welcome to the Rust AI Mentor!**                     |
|                                                                          |
|          *A focused, terminal-based guide to your Rust journey.*         |
|                                                                          |
|                                                                          |
|           **Please select your current Rust skill level:**               |
|                                                                          |
|                          Level 1: Just Starting                          |
|                          Level 2: Basic Syntax                           |
|                          Level 3: Ownership & Structs                    |
|                        > Level 4: Enums & Matching                       |
|                          Level 5: Intermediate                           |
|                          Level 6: Traits & Generics                      |
|                                                                          |
|                                                                          |
|                        [ Press Enter to Begin ]                          |
|                                                                          |
+--------------------------------------------------------------------------+
| `(k/↑, j/↓)` Change Level `(?)` Help                             `(q)` Quit |
+--------------------------------------------------------------------------+
```

### **1.5. Index Selection View**

This screen appears after the user selects their skill level. It allows them to choose which content source they want to learn from.

```markdown
+--------------------------------------------------------------------------+
| **Rust AI Mentor v0.1.0 :: Level 4**                                     |
+--------------------------------------------------------------------------+
|                                                                          |
|                **Please select a content source:**                       |
|                                                                          |
|                                                                          |
|                      Rust Library                                        |
|                      Rust By Example                                     |
|                    > The Rust Programming Language                       |
|                      Random                                              |
|                                                                          |
|                                                                          |
|                                                                          |
|                                                                          |
|                    [ Press Enter to Continue ]                           |
|                                                                          |
+--------------------------------------------------------------------------+
| `(k/↑, j/↓)` Change Source `(?)` Help                           `(q)` Quit |
+--------------------------------------------------------------------------+
```

### **2. Loading View**

This view is displayed during asynchronous operations, such as when the application is calling the LLM API to fetch a new learning module. It provides clear feedback that the app is working.

```markdown
+--------------------------------------------------------------------------+
| **Rust AI Mentor :: Level 4**                                            |
+--------------------------------------------------------------------------+
|                                                                          |
|                                                                          |
|                                                                          |
|                                                                          |
|                                                                          |
|                  Generating your learning module... ⏳                   |
|                                                                          |
|                                                                          |
|                                                                          |
|                                                                          |
|                                                                          |
|                                                                          |
|                                                                          |
+--------------------------------------------------------------------------+
|                                Please wait...                            |
+--------------------------------------------------------------------------+
```

### **3. Main Learning View**

This is the core view of the application, displaying the AI-generated content. It is designed for maximum readability, with clear sections and syntax highlighting for code.

```markdown
+--------------------------------------------------------------------------+
| **Rust AI Mentor :: Level 4**                                            |
+--------------------------------------------------------------------------+
|                                                                          |
| ## TOPIC: Understanding Ownership                                        |
|                                                                          |
| > **Explanation:**                                                       |
| > In Rust, every value has a variable that’s called its owner. There can |
| > only be one owner at a time. When the owner goes out of scope, the     |
| > value will be dropped. This rule is checked at compile time.           |
|                                                                          |
| ---                                                                      |
|                                                                          |
| **Code Snippets:**                                                       |
|                                                                          |
| ```rust
| // 1. Ownership and Scope
| {
|     let s = String::from("hello"); // s is valid from this point forward
|     // do stuff with s
| } // this scope is now over, and s is no longer valid
| ```
|                                                                          |
| ---                                                                      |
|                                                                          |
| **Exercises:**                                                           |
|                                                                          |
| 1. What is the error in the code below? Explain why it happens based on  |
|    the concept of 'move'.                                                |
|    ```rust
|    let s1 = String::from("hello");
|    let s2 = s1;
|    println!("{}, world!", s1);
|    ```
|                                                                          |
+--------------------------------------------------------------------------+
| `(n)` New Module | `(k/↑, j/↓)` Scroll | `(?)` Help | `(q)` Quit          |
+--------------------------------------------------------------------------+
```

### **4. Help Modal (Overlay)**

This modal is displayed as an overlay on top of the current view when the user presses `?`. It provides a quick reference for all available keybindings.

```markdown
+--------------------------------------------------------------------------+
| **Rust AI Mentor :: Level 4**                                            |
+------------------------------------------------------------------+-------+
|                                                                  |       |
| ## TOPIC: Understanding Ownership                              |       |
|                                                                  |       |
| > **Explanation:**                                               |       |
| > In Rust, every value has a variab┌───────────────────────────┐ |       |
| > only be one owner at a time. Whe │      **Keybindings**        │ |       |
| > value will be dropped. This rule ├───────────────────────────┤ |       |
|                                    │ `n`: Request New Module     │ |       |
| ---                                │ `q`: Quit Application       │ |       |
|                                    │ `?`: Toggle this Help menu  │ |       |
| **Code Snippets:**                 │ `k,↑`: Scroll Up            │ |       |
|                                    │ `j,↓`: Scroll Down          │ |       |
| ```rust                           │ `Esc`: Close this menu      │ |       |
| // 1. Ownership and Scope        │                           │ |       |
| {                                │ [ Press Esc to Close ]      │ |       |
|     let s = String::from("hello"); └───────────────────────────┘ |       |
|     // do stuff with s                                          |       |
| } // this scope is now over...                                   |       |
+------------------------------------------------------------------+-------+
| `(n)` New Module | `(k/↑, j/↓)` Scroll | `(?)` Help | `(q)` Quit          |
+--------------------------------------------------------------------------+
```

### **5. Quit Confirmation Modal (Overlay)**

This simple modal appears when the user presses `q` from a primary view, preventing accidental termination of the application.

```markdown
+--------------------------------------------------------------------------+
| **Rust AI Mentor :: Level 4**                                            |
+------------------------------------------------------------------+-------+
|                                                                  |       |
| ## TOPIC: Understanding Ownership                              |       |
|                                                                  |       |
| > **Explanation:**                                               |       |
| > In Rust, every value has a variable that’s called its owner. T |       |
| > only be one owner at a time.     ┌───────────────────────────┐ |       |
|                                    │ Are you sure you want to? │ |       |
| ---                                │           quit?           │ |       |
|                                    │                           │ |       |
| **Code Snippets:**                 │      **[ Yes ]**   [ No ]     │ |       |
|                                    └───────────────────────────┘ |       |
| ```rust                           |                           | |       |
| // 1. Ownership and Scope        |                           | |       |
| {                                |                           | |       |
|     let s = String::from("hello"); |                           | |       |
|     // do stuff with s                                          |       |
| } // this scope is now over...                                   |       |
+------------------------------------------------------------------+-------+
| `(n)` New Module | `(k/↑, j/↓)` Scroll | `(?)` Help | `(q)` Quit          |
+--------------------------------------------------------------------------+
```
