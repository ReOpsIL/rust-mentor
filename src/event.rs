// src/event.rs
use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, KeyCode};
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
}

pub struct EventHandler {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
}

impl EventHandler {
    pub fn new(tick_rate_ms: u64) -> Self {
        let (sender, receiver) = mpsc::channel(100);
        let tick_rate = Duration::from_millis(tick_rate_ms);
        let event_sender = sender.clone();

        tokio::spawn(async move {
            loop {
                if event::poll(tick_rate).unwrap_or(false) {
                    if let Ok(CrosstermEvent::Key(key)) = event::read() {
                        event_sender.send(Event::Key(key)).await.ok();
                    }
                }
                event_sender.send(Event::Tick).await.ok();
            }
        });

        Self { sender, receiver }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.receiver.recv().await.ok_or_else(|| anyhow::anyhow!("Event channel closed"))
    }
}