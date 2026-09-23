// src/event.rs
use anyhow::{Result, anyhow};
use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEvent};
use futures_util::StreamExt;
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Resize,
}

/// Merges terminal input (read asynchronously) with a periodic tick
pub struct EventHandler {
    receiver: mpsc::Receiver<Event>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::channel(100);

        tokio::spawn(async move {
            let mut reader = EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);
            tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            loop {
                let event = tokio::select! {
                    _ = tick.tick() => Event::Tick,
                    maybe_event = reader.next() => match maybe_event {
                        Some(Ok(CrosstermEvent::Key(key))) => Event::Key(key),
                        Some(Ok(CrosstermEvent::Resize(_, _))) => Event::Resize,
                        Some(Ok(_)) => continue,
                        Some(Err(err)) => {
                            tracing::error!("Failed to read terminal input: {}", err);
                            break;
                        }
                        None => break,
                    },
                };
                if sender.send(event).await.is_err() {
                    break; // The app has shut down
                }
            }
        });

        Self { receiver }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.receiver.recv().await.ok_or_else(|| anyhow!("Terminal input closed"))
    }
}
