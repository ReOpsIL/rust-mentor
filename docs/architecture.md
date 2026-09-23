# RustMentor — Architecture

RustMentor is a single-binary terminal application built on `ratatui` + `crossterm`, running on a `tokio` runtime.
All LLM calls go to OpenRouter over `reqwest` (streaming chat completions).

## Main loop

```
             ┌────────────── EventHandler (event.rs) ──────────────┐
terminal ──► │ crossterm EventStream (async) + 100 ms tick interval │ ──► Event::{Key, Tick, Resize}
             └─────────────────────────────────────────────────────┘                │
                                                                                     ▼
main.rs: loop { draw(&app); select! { event => app.handle_key_event / tick,   task_result => app.handle_task_result } }
                                                                                     ▲
background tasks (tokio::spawn / spawn_blocking) ── TaskResult over one mpsc channel ─┘
```

- The UI never blocks: every LLM request and every Cargo project creation runs in a background task.
- Each LLM request gets an id. `TaskResult`s carry the id, and the app ignores results of requests that were
  cancelled (Esc) or superseded — no stale content.
- Streamed text arrives as `TaskResult::Progress` and is shown as a live preview on the loading screen.
- A panic hook and a `Drop` guard on `Tui` restore the terminal. Logs go to a file, never to the terminal.

## Modules

| Module | Responsibility |
|---|---|
| `main.rs` | Startup (logging, config, terminal), main loop |
| `tui.rs` | Terminal setup/restore, panic hook |
| `event.rs` | Async terminal input + ticks |
| `app.rs` | Application state machine (`AppState`), key handling, background request management, history, scrolling, model picker state |
| `config.rs` | `Config` (serde, all fields defaulted), enums with `strum` (`Display`, `EnumIter`) and the `Cycle` trait, `ConfigService` (load/migrate/save) |
| `settings.rs` | Declarative settings screen: sections and items (label, value getter, change function) |
| `data.rs` | Embedded topic indexes (`include_str!`) and topic selection by level |
| `prompts.rs` | Prompt templates for learning modules, questions and applications |
| `llm.rs` | OpenRouter client: streaming completions (SSE), retries on 429/5xx, model list |
| `parsing.rs` | Lenient parsers for the `<<<marker>>>` response formats (tested against fixtures in `tests/fixtures/`) |
| `question_generator.rs` | Question / application types, answer handling, generation via `llm` |
| `model.rs` | Domain types: `LearningModule`, `CodeSnippet`, `Exercise`, resources |
| `resources.rs` | Links to documentation, forums, crates.io and GitHub for a topic |
| `cargo_project.rs` | Writes modules and applications as Cargo projects |
| `ui/` | Rendering: `mod.rs` (layout, title bar, key hints, helpers), `text.rs` (wrapping, Markdown, syntax highlighting), one module per screen, `modals.rs` (loading, help, quit, errors, status line) |

## State machine

```
Welcome ──Enter──► IndexSelection ──Enter──► QuestionGeneration ──► QuestionAnswering
   ▲                    │ (level < 3 and Library: LevelTooLowPopup)          │ Enter (all answered)
   │                    ▼                                                    ▼
   └────Esc──── Learning ◄──── Loading ◄──Esc/Enter── ApplicationDisplay ◄── ApplicationGeneration
                 │  n: new topic → Loading        (module is generated in the background
                 │  w: questions for this topic    while the application is shown)
                 └─ s (any non-busy screen): Settings, Esc returns to the previous screen
```

If application generation is disabled in the settings, answering the questions leads straight to the learning
module.

## Rendering

Views take `&App` and return their key hints. Laying out a learning module (Markdown + syntax highlighting) is
cached per module version and width, and only the visible lines are drawn. Scroll limits measured during rendering
are stored in `Cell`s on `App::*_scroll`, so key handling can clamp scrolling.

## LLM response formats

Responses use line-based markers instead of JSON (many free models don't support `response_format`, and code
inside JSON strings is error-prone):

- Learning module: `<<<explanation: title>>>`, `<<<code_snippet n: title>>>` (first line `// code snippet: …`),
  `<<<exercise n: name>>>` (first line `// exercise description: …`)
- Questions: `<<<question:n>>>`, `[TYPE: multiple|binary]`, `[OPTIONS: (1) … (4) …]`, `<<<end>>>`
- Application: `<<<application_name>>>`, `<<<application_description>>>`, `<<<application_features>>>`,
  `<<<code_snippet:title>>>`, each closed by `<<<end>>>`

The parsers tolerate code fences, missing headers and extra text. If a learning module can't be parsed, the raw
response is shown as the explanation.
