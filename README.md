# RustMentor

RustMentor is a terminal application that helps you learn Rust with AI-generated material tailored to your skill
level. Pick a level and a content source; RustMentor asks a few questions about the kind of program you'd like to
build, generates a small Rust application from your answers, and prepares a learning module (explanation, runnable
examples and exercises) on the same topic. Anything it generates can be written out as a Cargo project.

It uses [OpenRouter](https://openrouter.ai), so any model available there can be used (a free model is the default).

## Features

- **10 skill levels**, from absolute beginner to expert
- **Content sources**: Rust library topics, *Rust By Example*, *The Rust Programming Language*, a **Cyber**
  security-engineering track (memory safety, cryptography, hardening, reverse engineering and defensive malware
  analysis), or a random mix of the general Rust sources — choose a topic from a searchable list filtered to your
  level, let RustMentor pick one at random, or type your own
- **Questions → application → learning module** flow, with streaming output and a live preview while generating
- **Learning modules** with Markdown explanations, syntax-highlighted examples, exercises with starter code and
  links to documentation, forums, crates.io and GitHub
- **Cargo projects**: examples become `cargo run --example …` targets, exercises become binaries
- **History**: browse the modules generated in this session
- **Settings**: code complexity, explanation verbosity, focus area, 40+ learning goals (web development, embedded,
  machine learning, …), question style and count, and a model picker listing all OpenRouter models

## Installation

Requirements: Rust (edition 2024, Rust 1.88 or newer), an [OpenRouter API key](https://openrouter.ai/keys) and a
terminal with UTF-8 and color support.

```bash
git clone git@github.com:ReOpsIL/rust-mentor.git
cd rust-mentor
cargo install --path .        # or: cargo build --release

export OPENROUTER_API_KEY="your_api_key_here"
rust-mentor                   # or: cargo run --release
```

## How to use

1. **Select your skill level** with ↑/↓ (or `j`/`k`, or `1`-`9` and `0` for 10) and press Enter.
2. **Select a content source, then a topic.** A searchable list shows the source's topics for your level: type to
   filter, ↑/↓ to move, Enter to choose. The first entry, **Random topic**, lets RustMentor pick one; if you type
   something, the last entry turns your text into a **custom topic** (e.g. "lifetimes in async code").
3. **Answer the questions** about the application you'd like to build: `y`/`n` for yes/no questions, `1`-`4` (or
   `a`-`d`) for multiple choice. `←`/`→` move between questions; Enter generates the application once all are
   answered.
4. **Review the generated application.** Enter (or `c`) writes it as a Cargo project; Esc continues. The learning
   module for the topic is generated in the background meanwhile.
5. **Study the learning module.** Scroll with ↑/↓, PgUp/PgDn, `g`/`G`. `n` generates a module on a random topic,
   `t` opens the topic list to choose the next one, `w` asks questions about the current topic, `c` writes the
   module as a Cargo project, `[` and `]` browse history.

Everywhere: `?` shows all keybindings, `s` opens the settings, Esc goes back (or cancels a running request), `q`
quits.

In the settings, Tab switches sections, ↑/↓ selects an option and ←/→ or Enter changes it. The **AI Model** section
opens a searchable list of OpenRouter models (free models first).

## Configuration

Settings are saved automatically to `config.toml` in the configuration directory:

| Platform | Location |
|---|---|
| macOS | `~/Library/Application Support/rust-mentor/config.toml` |
| Linux | `~/.config/rust-mentor/config.toml` |

Settings from `~/rust-mentor.conf` (used by earlier versions) are migrated on first start. Besides the options in
the settings screen, the file has a `projects_dir` option: the directory where Cargo projects are created (default:
the current directory; `~` is expanded).

Logs are written to `rust-mentor.log` in the data directory (`~/Library/Application Support/rust-mentor/` on macOS,
`~/.local/share/rust-mentor/` on Linux), never to the terminal.

## Development

```bash
cargo test                                   # unit tests, including parser fixtures and UI render tests
cargo clippy --all-targets -- -D warnings
cargo fmt
```

CI runs the same checks on Linux and macOS. The code layout is described in
[docs/architecture.md](docs/architecture.md); planned and completed work is tracked in
[docs/improvement_plan.md](docs/improvement_plan.md).

## License

MIT — see [LICENSE](LICENSE).
