// src/main.rs
mod app;
mod cargo_project;
mod config;
mod data;
mod event;
mod llm;
mod model;
mod parsing;
mod prompts;
mod question_generator;
mod resources;
mod settings;
mod tui;
mod ui;

use anyhow::Result;
use app::{App, TaskResult};
use config::ConfigService;
use event::{Event, EventHandler};
use std::env;
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;
use tui::Tui;

#[tokio::main]
async fn main() -> Result<()> {
    // Log to a file - writing to stdout would corrupt the TUI
    let log_path = init_logging();

    // Read OPENROUTER_API_KEY environment variable
    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        tracing::warn!("OPENROUTER_API_KEY environment variable not set");
        String::new()
    });

    // Load the settings before taking over the terminal, so errors are readable
    let config_service = match ConfigService::load() {
        Ok(config_service) => config_service,
        Err(err) => {
            eprintln!("RustMentor could not load its settings: {err:#}");
            std::process::exit(1);
        }
    };

    // Create the application state
    let (mut app, mut task_receiver) = App::new(api_key, config_service);

    // Restore the terminal if anything panics
    tui::install_panic_hook();

    // Initialize the terminal user interface
    let mut tui = Tui::new()?;
    tui.enter()?;

    let result = run(&mut app, &mut tui, &mut task_receiver).await;

    // Restore the terminal (also done on drop, but report errors here)
    tui.exit()?;

    if let Err(err) = &result {
        eprintln!("RustMentor exited with an error: {err:#}");
        if let Some(path) = log_path {
            eprintln!("See the log file for details: {}", path.display());
        }
    }
    result
}

async fn run(app: &mut App, tui: &mut Tui, task_receiver: &mut mpsc::UnboundedReceiver<TaskResult>) -> Result<()> {
    // Create an event handler
    let mut event_handler = EventHandler::new(Duration::from_millis(100));

    // Start the main loop
    while app.is_running {
        // Render the UI
        tui.draw(app)?;
        // Handle terminal events and results of background tasks
        tokio::select! {
            event = event_handler.next() => match event? {
                Event::Tick => app.tick(),
                Event::Key(key_event) => app.handle_key_event(key_event)?,
                Event::Resize => {} // Redrawn on the next iteration
            },
            Some(task_result) = task_receiver.recv() => app.handle_task_result(task_result),
        }
    }
    Ok(())
}

/// Sets up file logging. Returns the log file path, or None if logging is disabled.
fn init_logging() -> Option<PathBuf> {
    let log_dir = directories::ProjectDirs::from("", "", "rust-mentor")
        .map(|dirs| dirs.data_local_dir().to_path_buf())
        .unwrap_or_else(env::temp_dir);
    std::fs::create_dir_all(&log_dir).ok()?;
    let log_path = log_dir.join("rust-mentor.log");
    let log_file = std::fs::OpenOptions::new().create(true).append(true).open(&log_path).ok()?;

    tracing_subscriber::fmt()
        .with_writer(std::sync::Mutex::new(log_file))
        .with_ansi(false)
        .with_max_level(tracing::Level::INFO)
        .init();

    Some(log_path)
}
