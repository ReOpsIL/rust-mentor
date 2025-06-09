# RustMentor

RustMentor is an interactive terminal-based application designed to help users learn Rust programming language through personalized, AI-generated tutorials. It uses language models to create custom learning modules tailored to your skill level.

## Features

- **Personalized Learning**: Choose from 10 different skill levels, from absolute beginner to expert
- **AI-Generated Content**: Uses OpenRouter API (with Google's Gemini 2.5 Pro) to generate custom learning modules
- **Interactive Terminal UI**: Easy-to-navigate text-based interface
- **Comprehensive Learning Modules**:
  - Detailed explanations in Markdown format
  - Runnable code examples with comments
  - Practice exercises to reinforce learning
- **Random Topic Selection**: Automatically selects topics from "Rust by Example" and "The Rust Programming Language" books based on your level
- **Content Customization**:
  - Adjust code complexity (Simple, Moderate, Complex)
  - Control explanation verbosity (Concise, Moderate, Detailed)
  - Set focus area (Concepts, Code Examples, Exercises, Balanced)
- **Learning Resources Management**:
  - Toggle visibility of official documentation
  - Control display of community resources
  - Show/hide crates.io packages
  - Enable/disable GitHub repository suggestions
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

## How to Use

1. **Start the application**:
   ```bash
   cargo run --release
   ```

2. **Select your skill level** (1-10) using the up/down arrow keys or 'j'/'k' and press Enter.

3. **Navigate the learning module**:
   - Scroll up/down: Arrow keys or 'j'/'k'
   - Generate a new module: 'n'
   - Access settings: 's'
   - Return to level selection: Esc
   - Show help: '?'
   - Quit: 'q'

4. **Customize your settings**:
   - Navigate between settings sections: Tab
   - Navigate options: Arrow keys or 'j'/'k'
   - Toggle or cycle selected option: Enter/Space
   - Return to previous screen: Esc

5. **Confirm quit**: Use left/right arrow keys to select Yes/No and press Enter.

## Requirements

- Rust and Cargo installed
- OpenRouter API key (sign up at [openrouter.ai](https://openrouter.ai))
- Terminal with support for TUI applications

## Dependencies

- ratatui and crossterm for the terminal UI
- tokio for async runtime
- reqwest for API calls
- serde for JSON serialization/deserialization
- syntect for syntax highlighting
- anyhow for error handling

## License

[MIT License](LICENSE)
