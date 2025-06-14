# RustMentor

RustMentor is an interactive terminal-based application designed to help users learn Rust programming language through personalized, AI-generated tutorials. It uses language models to create custom learning modules tailored to your skill level.

## Features

- **Personalized Learning**: Choose from 10 different skill levels, from absolute beginner to expert
- **AI-Generated Content**: Uses OpenRouter API (with Google's Gemma 3 model) to generate custom learning modules
- **Interactive Terminal UI**: Easy-to-navigate text-based interface
- **Comprehensive Learning Modules**:
  - Detailed explanations in Markdown format
  - Runnable code examples with comments
  - Practice exercises to reinforce learning
- **Multiple Content Sources**:
  - Rust Library (standard and community libraries)
  - Rust by Example
  - The Rust Programming Language book
  - Random selection from all sources
- **Content Customization**:
  - Adjust code complexity (Simple, Moderate, Complex)
  - Control explanation verbosity (Concise, Moderate, Detailed)
  - Set the focus area (Concepts, Code Examples, Exercises, Balanced)
  - Choose from over 40 specialized learning goals (Web Development, Systems Programming, Machine Learning, etc.)
- **Learning Resources Management**:
  - Toggle visibility of official documentation
  - Control the display of community resources
  - Show/hide crates.io packages
  - Enable/disable GitHub repository suggestions
- **Question Generator**:
  - Generate quiz questions based on the current learning module
  - Answer binary (Yes/No) or multiple-choice questions
  - Customize the number of questions and question types
- **Application Generator**:
  - Create sample Rust applications based on answered questions
  - Automatically generate Cargo projects for learning modules and applications
- **Settings Management**: Dedicated settings screen for customizing your learning experience

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/RustMentor.git
   cd RustMentor
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Set up your OpenRouter API key:
   ```bash
   export OPENROUTER_API_KEY="your_api_key_here"
   ```

   The application will create a configuration file at `~/rust-mentor.conf` on first run.

## How to Use

1. **Start the application**:
   ```bash
   cargo run --release
   ```

2. **Select your skill level** (1-10) using the up/down arrow keys or 'j'/'k' and press Enter.

3. **Select a content source**:
   - Rust Library (standard and community libraries)
   - Rust By Example
   - The Rust Programming Language
   - Random (selects from any of the above sources)

4. **Navigate the learning module**:
   - Scroll up/down: Arrow keys or 'j'/'k'
   - Generate a new module: 'n'
   - Generate questions: 'w'
   - Access settings: 's'
   - Return to level selection: Esc
   - Show help: '?'
   - Quit: 'q'

5. **Answer questions**:
   - Navigate between questions: Left/Right arrow keys or 'h'/'l'
   - For binary questions: 'y' for Yes, 'n' for No
   - For multiple-choice questions: '1'-'4' or 'a'-'d'
   - Generate application (after answering all questions): Enter
   - Return to learning module: Esc

6. **View generated application**:
   - Create Cargo project from application: Enter
   - Return to learning module: Esc

7. **Customize your settings**:
   - Navigate between settings sections: Tab
   - Navigate options: Arrow keys or 'j'/'k'
   - Toggle or cycle selected option: Left/Right arrow keys or 'j'/'k'
   - Return to previous screen: Esc

8. **Confirm quit**: Use left/right arrow keys to select Yes/No and press Enter.

## Requirements

- Rust and Cargo installed
- OpenRouter API key (sign up at [openrouter.ai](https://openrouter.ai))
- Terminal with support for TUI applications

## Dependencies

- ratatui and crossterm for the terminal UI
- tokio for async runtime
- reqwest for API calls
- serde and serde_json for JSON serialization/deserialization
- syntect for syntax highlighting
- anyhow for error handling
- tracing and tracing-subscriber for logging
- toml and toml_edit for configuration management
- directories for finding user directories
- textwrap for text formatting
- lazy_static for lazy initialization
- chrono for date and time handling
- rand for random number generation
- regex for regular expressions

## License

[MIT License](LICENSE)
