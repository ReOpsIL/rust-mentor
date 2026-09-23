# RustMentor — Review & Improvement Plan

_Review date: 2026-09-23. Baseline: builds, 27 clippy warnings, 0 tests._

## Verified bugs

| # | Issue | Where | Impact |
|---|---|---|---|
| 1 | Logging goes to **stdout** while the TUI is drawn | `main.rs` | Any `tracing::info!/error!` garbles the screen |
| 2 | Terminal not restored on panic / error | `main.rs` | `?` in the loop or any panic leaves the shell in raw mode + alt screen |
| 3 | Panics on narrow terminals | `ui.rs` (`width as usize - 4`) | usize underflow |
| 4 | Learning module is one request stale | `app.rs` tick / `generate_learning_module` | After app generation a module request is fired, but the receiver is polled only in `Loading`, so the next `n` shows the *previous* result |
| 5 | Pressing `s` during generation loses the result | `app.rs` | Receivers only polled in their matching state |
| 6 | Data files loaded via relative path | `data.rs` | App only works when run from repo root |
| 7 | Snippet descriptions never parsed | `llm.rs` prompt vs `prompt_response.rs` | Prompt asks for `// code snippet:`, parser looks for `# code snippet:` |
| 8 | 4th multiple-choice option dropped | `question_generator.rs` | `ends_with("]")` check runs before option parsing |
| 9 | Regex `.unwrap()` can panic; regex compiled per line | `question_generator.rs` | Crash on `(1)Text` |
| 10 | Code fences stripped from explanations | `prompt_response.rs` | Markdown loses fences |
| 11 | Blocking `cargo init` on UI thread, on every module | `app.rs` tick | UI freeze; clutters CWD; fails on same topic same day |
| 12 | Settings keys don't match help | `app.rs` vs `ui.rs` | Nav is `i`/`m`, change is `j`/`k`; help says j/k nav, Enter/Space toggle (no-op) |
| 13 | `Config::load().unwrap()` on every LLM call | `llm.rs` | Disk re-read; panic kills task |
| 14 | No HTTP timeout | `llm.rs` | Can hang forever in "Loading…", no cancel |
| 15 | `num_questions` changes not saved | `config.rs` | Lost on restart |
| 16 | No `#[serde(default)]` on config | `config.rs` | Any new field breaks existing configs → `process::exit(-1)` |

Smaller: wrong help text ("Press 'q' to generate questions"), Settings Esc always returns to Welcome, markdown highlighted as Rust, full re-highlight every 80 ms tick, tick-rate comment wrong.

## Decisions (defaults taken)

1. **Flow** — keep the current flow (level → source → questions → application → learning module), which the last commit introduced deliberately. The topic is now picked from the selected source *before* questions, so questions and the follow-up module share that topic.
2. **Cargo project creation** — opt-in via a key (`c`) on the Learning screen instead of automatic after every module; runs off the UI thread.
3. **LLM output format** — decide in Phase 3 (JSON via `response_format` vs hardened `<<<…>>>` parser).

## Progress

- **2026-09-23 (1)** — Phases 1 and 2 implemented and verified in a pseudo-terminal.
- **2026-09-23 (2)** — Phases 3–7 implemented. 48 unit tests (parsers against fixtures, config migration,
  settings, App state machine, Cargo project writer, text layout, render smoke tests of every screen at sizes down
  to 0×0). `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` pass; the same checks
  run in CI. Verified in a pseudo-terminal: start from any directory, legacy config migration, error popup without
  an API key, help/settings, resize to a tiny terminal, clean exit.
  _Not yet verified against the live OpenRouter API (needs an API key)._

## Phases

### Phase 1 — Stop crashes & screen corruption
- [x] Log to a file instead of stdout
- [x] Panic hook + `Drop` guard restores the terminal
- [x] No width underflow in the UI (render tests down to 0×0)
- [x] Embed data files with `include_str!`
- [x] Remove `unwrap()`s in LLM / question paths
- [x] Fix dropped 4th multiple-choice option

### Phase 2 — State machine & flow
- [x] Single task-result channel consumed in `main` via `tokio::select!` (fixes #4, #5)
- [x] Request IDs so stale / cancelled results are ignored
- [x] Error popup instead of fake "Error" learning modules
- [x] Esc cancels Loading / generation (aborts the task)
- [x] Settings: remember previous screen; consistent keys; not reachable while generating
- [x] Cargo project creation opt-in (`c`), non-blocking, unique directory names
- [x] Topic chosen from the selected source before questions

### Phase 3 — LLM layer robustness & tests
- [x] Config loaded once; `#[serde(default)]` everywhere; stored in the platform config dir
      (`ProjectDirs`), `~/rust-mentor.conf` migrated; readable error before the TUI starts if invalid
- [x] Timeouts (connect 15 s, read 90 s) and up to 3 attempts on 429/5xx/connection errors (honours `Retry-After`)
- [x] OpenRouter errors reported inside 200 responses / streams are surfaced
- [x] Parsers rewritten in `parsing.rs` (#7 description marker, #10 fences, titles with colons, indentation kept
      in application code, lenient question options), tested against fixtures in `tests/fixtures/`
- [x] Answers `a`–`d` map to options 1–4; invalid keys are ignored
- [x] **Decision:** keep the `<<<marker>>>` format rather than JSON output — many free OpenRouter models don't
      support `response_format`, and code inside JSON strings is error-prone. Documented in `parsing.rs` and
      `docs/architecture.md`
- [x] Deleted `src/bin/test_parse_response.rs`

### Phase 4 — Code quality
- [x] `strum` for all setting enums + generic `Cycle` trait (removed ~250 lines of manual cycling/Display)
- [x] Declarative settings (`settings.rs`) drive both key handling and rendering
- [x] `ui.rs` split into `ui/{mod,text,welcome,learning,settings,questions,application,modals}.rs`
- [x] Prompts (`prompts.rs`), parsing (`parsing.rs`), domain types (`model.rs`) and resources (`resources.rs`)
      moved out of `app.rs` / `llm.rs`
- [x] Markdown rendering for explanations (headings, lists, quotes, inline code/bold, highlighted fences); code
      blocks sized to the terminal width
- [x] Layout/highlighting cached per content version and width; only visible lines drawn
- [x] Dead code removed; zero clippy warnings (`-D warnings`)
- [x] Bounded scrolling with PgUp/PgDn/`b`/Space/`g`/`G`, scroll position indicator
- [x] Cargo projects written directly (no `cargo init` subprocess); examples/exercises always get a `main`

### Phase 5 — Dependencies
- [x] ratatui 0.30, crossterm 0.29, reqwest 0.13, toml 1.x (single TOML crate), strum 0.28
- [x] `lazy_static` → `std::sync::LazyLock`
- [x] crossterm `EventStream` instead of blocking `poll` inside `tokio::spawn`
- [x] Package renamed to `rust-mentor` (binary `rust-mentor`), `rust-version = "1.88"`, `Cargo.lock` tracked

### Phase 6 — Repo & docs hygiene
- [x] `.DS_Store` and `ignore/` untracked (kept locally) and ignored
- [x] `docs/tasks.md`, `docs/tasks_scr.md` moved to `docs/archive/`
- [x] README rewritten (flow, keys, configuration, logs, development); `docs/architecture.md` rewritten
- [x] CI (`.github/workflows/ci.yml`): fmt, clippy `-D warnings`, tests on Linux and macOS
- [x] MIT `LICENSE` file added

### Phase 7 — Features
- [x] Skill level passed into question and application prompts; the question style setting (mixed / yes-no /
      multiple choice) is honoured; "Generate Application" toggle skips straight to the learning module
- [x] Application view: scrolling, all snippets, Markdown description
- [x] Streaming responses with a live preview and character count while generating
- [x] Model picker in Settings (searchable OpenRouter model list, free models first)
- [x] Session history of learning modules (`[` / `]`)
- [x] Answering a question jumps to the next unanswered one; level can be picked with digit keys
- [x] **Cyber** content source: a security-engineering curriculum (`data/cyber_security.json`, 8 chapters) —
      vulnerability classes and how Rust prevents them, unsafe/low-level, cryptography, hardening, reverse
      engineering and defensive malware analysis/detection; opt-in (kept out of the Random pool)
- [x] Topic picker: after choosing a source (and with `t` on the learning screen), choose a topic from a
      searchable list for your level, a random topic, or a custom topic typed by the user
