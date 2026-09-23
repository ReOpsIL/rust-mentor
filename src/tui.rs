// src/tui.rs
use crate::{app::App, ui};
use anyhow::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{Stdout, stdout};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    entered: bool,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal, entered: false })
    }

    pub fn enter(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        self.entered = true;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        if self.entered {
            self.entered = false;
            restore_terminal()?;
        }
        Ok(())
    }

    pub fn draw(&mut self, app: &App) -> Result<()> {
        self.terminal.draw(|frame| ui::render(frame, app))?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = self.exit();
    }
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

/// Restores the terminal before the default panic message is printed,
/// so a panic doesn't leave the shell in raw mode on the alternate screen.
/// Panics in background tasks (worker threads) are only logged: tokio catches
/// them and the app reports them as task errors while the UI keeps running.
pub fn install_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        tracing::error!("panic: {info}");
        if std::thread::current().name() == Some("main") {
            let _ = restore_terminal();
            default_hook(info);
        }
    }));
}
