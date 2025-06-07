// src/main.rs
mod app;
mod data;
mod event;
mod tui;
mod ui;
mod components;
mod llm;

use anyhow::Result;
use app::App;
use event::{Event, EventHandler};
use tui::Tui;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Read OPENROUTER_API_KEY environment variable
    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        tracing::warn!("OPENROUTER_API_KEY environment variable not set");
        String::new()
    });

    // Create the application state
    let mut app = App::new(api_key);

    // Initialize the terminal user interface
    let mut tui = Tui::new()?;
    tui.enter()?;

    // Create an event handler
    let mut event_handler = EventHandler::new(250); // 250ms tick rate

    // Start the main loop
    while app.is_running {
        // Render the UI
        tui.draw(&mut app)?;
        // Handle events
        match event_handler.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => app.handle_key_event(key_event)?,
        }
    }

    // Restore the terminal
    tui.exit()?;
    Ok(())
}
