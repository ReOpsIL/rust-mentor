// src/app.rs
use anyhow::Result;
use crossterm::event::{KeyEvent, KeyCode};
use tokio::sync::mpsc;
use crate::data;
use crate::llm::LlmClient;

pub struct LearningModule {
    pub topic: String,
    pub explanation: String,
    pub code_snippets: Vec<String>,
    pub exercises: Vec<String>,
}

pub enum AppState {
    Welcome,
    Learning,
    Loading,
}

pub struct App {
    pub is_running: bool,
    pub current_state: AppState,
    pub selected_level: u8,
    pub show_help: bool,
    pub show_quit_confirmation: bool,
    pub quit_confirmation_selected: bool, // true = Yes, false = No
    pub api_key: String,
    pub scroll_offset: u16,
    pub current_module: Option<LearningModule>,
    llm_client: LlmClient,
    module_receiver: mpsc::Receiver<Result<LearningModule>>,
    module_sender: mpsc::Sender<Result<LearningModule>>,
}

impl App {
    pub fn new(api_key: String) -> Self {
        // Create a channel for communicating between the LLM task and the main app
        let (module_sender, module_receiver) = mpsc::channel(10);

        Self {
            is_running: true,
            current_state: AppState::Welcome,
            selected_level: 5, // Default level
            show_help: false,
            show_quit_confirmation: false,
            quit_confirmation_selected: false, // Default to "No"
            llm_client: LlmClient::new(api_key.clone()),
            api_key,
            scroll_offset: 0,
            current_module: None,
            module_receiver,
            module_sender,
        }
    }

    pub fn tick(&mut self) {
        // Check if we're in the Loading state and if there's a message from the LLM client
        if let AppState::Loading = self.current_state {
            // Try to receive a message from the channel (non-blocking)
            match self.module_receiver.try_recv() {
                Ok(result) => {
                    match result {
                        Ok(module) => {
                            // Successfully received a module, update the state
                            self.current_module = Some(module);
                            self.current_state = AppState::Learning;
                            self.scroll_offset = 0; // Reset scroll position for new content
                        },
                        Err(err) => {
                            // There was an error generating the module
                            tracing::error!("Failed to generate learning module: {}", err);

                            // Create an error module
                            let error_module = LearningModule {
                                topic: "Error Generating Content".to_string(),
                                explanation: format!("There was an error generating content: {}\n\nPlease try again or select a different level.", err),
                                code_snippets: vec![],
                                exercises: vec![],
                            };

                            self.current_module = Some(error_module);
                            self.current_state = AppState::Learning;
                        }
                    }
                },
                Err(mpsc::error::TryRecvError::Empty) => {
                    // No message yet, continue waiting
                },
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    // Channel is disconnected, this shouldn't happen in normal operation
                    tracing::error!("Module channel disconnected");

                    // Create an error module
                    let error_module = LearningModule {
                        topic: "Communication Error".to_string(),
                        explanation: "There was an error communicating with the content generation service.\n\nPlease try again or select a different level.".to_string(),
                        code_snippets: vec![],
                        exercises: vec![],
                    };

                    self.current_module = Some(error_module);
                    self.current_state = AppState::Learning;
                }
            }
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        // Global keybindings
        if self.show_help {
            if let KeyCode::Esc | KeyCode::Char('?') = key_event.code {
                self.show_help = false;
            }
            return Ok(());
        }

        if self.show_quit_confirmation {
            match key_event.code {
                KeyCode::Enter => {
                    if self.quit_confirmation_selected {
                        // User selected "Yes"
                        self.is_running = false;
                    } else {
                        // User selected "No"
                        self.show_quit_confirmation = false;
                    }
                },
                KeyCode::Left | KeyCode::Right | KeyCode::Char('h') | KeyCode::Char('l') => {
                    // Toggle between Yes and No
                    self.quit_confirmation_selected = !self.quit_confirmation_selected;
                },
                KeyCode::Esc | KeyCode::Char('q') => self.show_quit_confirmation = false,
                _ => {}
            }
            return Ok(());
        }

        match key_event.code {
            KeyCode::Char('q') => self.show_quit_confirmation = true,
            KeyCode::Char('?') => self.show_help = true,
            _ => {}
        }

        // Context-specific keybindings
        match self.current_state {
            AppState::Welcome => self.handle_welcome_keys(key_event),
            AppState::Learning => self.handle_learning_keys(key_event),
            _ => {}
        }
        Ok(())
    }

    fn handle_welcome_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Down | KeyCode::Char('j') => {
                self.selected_level = (self.selected_level + 1).min(10);
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.selected_level = (self.selected_level - 1).max(1);
            }
            KeyCode::Enter => {
                self.current_state = AppState::Loading;
                // Generate a learning module based on the selected level
                self.generate_learning_module();
            }
            _ => {}
        }
    }

    fn generate_learning_module(&mut self) {
        // Get a random topic based on the user's level
        match data::get_random_topic_for_level(self.selected_level) {
            Ok(topic) => {
                // Clone the sender and topic for the async task
                let sender = self.module_sender.clone();
                let topic_clone = topic.clone();
                let level = self.selected_level;
                let llm_client = self.llm_client.clone();

                // Spawn an async task to call the LLM
                tokio::spawn(async move {
                    // Call the LLM to generate a learning module
                    let result = llm_client.generate_learning_module(&topic_clone, level).await;

                    // Send the result back to the main thread
                    if let Err(e) = sender.send(result).await {
                        tracing::error!("Failed to send learning module: {}", e);
                    }
                });

                // The app remains in the Loading state until the async task completes
                // The tick method will handle the response when it arrives
            },
            Err(err) => {
                // If there was an error getting a topic, create an error module
                tracing::error!("Failed to get a random topic: {}", err);

                let module = LearningModule {
                    topic: "Error Loading Topic".to_string(),
                    explanation: format!("There was an error loading a topic for level {}. Please try again.", 
                                        self.selected_level),
                    code_snippets: vec![],
                    exercises: vec![],
                };

                // Set the current module
                self.current_module = Some(module);

                // Transition to the Learning state
                self.current_state = AppState::Learning;
            }
        }
    }

    fn handle_learning_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('n') => {
                // Request new module (F007)
                self.current_state = AppState::Loading;
                // Generate a new learning module
                self.generate_learning_module();
            }
            KeyCode::Esc => {
                self.current_state = AppState::Welcome;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                // Scroll up (if scroll_offset > 0)
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                // Scroll down (we don't know the max scroll, so we don't limit it)
                self.scroll_offset += 1;
            }
            _ => {}
        }
    }
}
