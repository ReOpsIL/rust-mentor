// src/app.rs
use crate::data;
use crate::llm::LlmClient;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use std::fmt;

// Define LearningGoal enum for personalized learning paths
#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Debug, )]
pub enum LearningGoal {
    ARVR, // Augmented Reality / Virtual Reality
    AsyncProgramming,
    BigData,
    Bioinformatics,
    Bitcoin,
    CloudComputing,
    ComputerVision,
    Concurrency,
    Cuda, // Compute Unified Device Architecture (for GPU programming)
    Cybersecurity,
    DICOM, // (Medical Imaging standard)
    DataScience,
    Databases,
    DeepLearning,
    DevOps,
    DistributedSystems,
    EdgeComputing,
    EmbeddedSystems,
    EthicalAI,
    GANs, // Generative Adversarial Networks (Deep Learning)
    GPU, // Graphics Processing Unit
    General,
    Graphics,
    HL7, // (Medical data exchange standard)
    ImageProcessing, // Often a component of Computer Vision
    MachineLearning,
    MedicalImaging,
    MicroServices,
    MicroVM,
    NaturalLanguageProcessing, // NLP
    Networking,
    OperatingSystems,
    PyTorch,
    QuantumComputing,
    ROS, // Robot Operating System
    ReinforcementLearning,
    Robotics,
    SLAM, // Simultaneous Localization and Mapping (Robotics, AR/VR)
    SensorFusion, // Relevant to Robotics and CV
    SystemsProgramming,
    TUI, // Terminal User Interface
    Transformers, // (Deep Learning, NLP)
    UserInterface,
    WebDevelopment,
}
// Macro to convert enum item to string
impl fmt::Display for LearningGoal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            LearningGoal::General => "General",
            LearningGoal::WebDevelopment => "Web Development",
            LearningGoal::SystemsProgramming => "Systems Programming",
            LearningGoal::DataScience => "Data Science",
            LearningGoal::MicroServices => "Microservices",
            LearningGoal::Bitcoin => "Bitcoin",
            LearningGoal::OperatingSystems => "Operating Systems",
            LearningGoal::MicroVM => "MicroVM",
            LearningGoal::Graphics => "Graphics",
            LearningGoal::UserInterface => "User Interface",
            LearningGoal::TUI => "TUI",
            LearningGoal::GPU => "GPU",
            LearningGoal::Cuda => "Cuda",
            LearningGoal::PyTorch => "PyTorch",
            LearningGoal::MachineLearning => "Machine Learning",
            LearningGoal::Networking => "Networking",
            LearningGoal::Databases => "Databases",
            LearningGoal::Concurrency => "Concurrency",
            LearningGoal::AsyncProgramming => "Async Programming",
            LearningGoal::ARVR => "AR/VR",
            LearningGoal::BigData => "Big Data",
            LearningGoal::Bioinformatics => "Bioinformatics",
            LearningGoal::CloudComputing => "Cloud Computing",
            LearningGoal::ComputerVision => "Computer Vision",
            LearningGoal::Cybersecurity => "Cybersecurity",
            LearningGoal::DICOM => "DICOM",
            LearningGoal::DeepLearning => "Deep Learning",
            LearningGoal::DevOps => "DevOps",
            LearningGoal::DistributedSystems => "Distributed Systems",
            LearningGoal::EdgeComputing => "Edge Computing",
            LearningGoal::EmbeddedSystems => "Embedded Systems",
            LearningGoal::EthicalAI => "Ethical AI",
            LearningGoal::GANs => "GANs",
            LearningGoal::HL7 => "HL7",
            LearningGoal::ImageProcessing => "Image Processing",
            LearningGoal::MedicalImaging => "Medical Imaging",
            LearningGoal::NaturalLanguageProcessing => "Natural Language Processing",
            LearningGoal::QuantumComputing => "Quantum Computing",
            LearningGoal::ROS => "ROS",
            LearningGoal::ReinforcementLearning => "Reinforcement Learning",
            LearningGoal::Robotics => "Robotics",
            LearningGoal::SLAM => "SLAM",
            LearningGoal::SensorFusion => "Sensor Fusion",
            LearningGoal::Transformers => "Transformers"
        };
        write!(f, "{}", display_str)
    }
}


use crate::prompt_response::{CodeSnippet, Exercise};
use crate::cargo_project;
use crate::config::ConfigService;

#[derive(Clone)]
pub struct LearningModule {
    pub topic: String,
    pub explanation: String,
    pub code_snippets: Vec<CodeSnippet>,
    pub exercises: Vec<Exercise>,
    pub additional_resources: Option<AdditionalResources>,
}

#[derive(Clone)]
pub struct AdditionalResources {
    pub official_docs: Vec<Resource>,
    pub community_resources: Vec<Resource>,
    pub crates_io: Vec<Resource>,
    pub github_repos: Vec<Resource>,
}

#[derive(Clone)]
pub struct Resource {
    pub title: String,
    pub url: String,
    pub description: String,
}

#[derive(PartialEq)]
pub enum AppState {
    Welcome,
    IndexSelection,
    Learning,
    Loading,
    LevelTooLowPopup,
    Settings,
    QuestionGeneration,
    QuestionAnswering,
    ApplicationGeneration,
    ApplicationDisplay,
}

pub enum IndexType {
    RustLibrary,
    RustByExample,
    RustProgrammingLanguage,
    Random,
}

pub struct App {
    pub is_running: bool,
    pub current_state: AppState,
    pub selected_level: u8,
    pub selected_index: IndexType,
    pub index_selection_cursor: usize, // 0 = RustLibrary, 1 = RustProgrammingLanguage, 2 = Random
    pub settings_cursor: usize, // Cursor position in settings screen
    pub settings_section: SettingsSection, // Current section in settings screen
    pub show_help: bool,
    pub show_quit_confirmation: bool,
    pub quit_confirmation_selected: bool, // true = Yes, false = No
    pub api_key: String,
    pub scroll_offset: u16,
    pub current_module: Option<LearningModule>,
    pub popup_start_time: Option<std::time::Instant>, // For tracking popup display time
    llm_client: LlmClient,
    module_receiver: mpsc::Receiver<Result<LearningModule>>,
    module_sender: mpsc::Sender<Result<LearningModule>>,
    config_service: ConfigService,
    // Question generator fields
    pub question_set: Option<crate::question_generator::QuestionSet>,
    pub generated_application: Option<crate::question_generator::GeneratedApplication>,
    pub question_generator: Option<crate::question_generator::QuestionGenerator>,
    pub question_generation_rx: Option<mpsc::UnboundedReceiver<Result<crate::question_generator::QuestionSet>>>,
    pub application_generation_rx: Option<mpsc::UnboundedReceiver<Result<crate::question_generator::GeneratedApplication>>>,
}

#[derive(PartialEq)]
pub enum SettingsSection {
    LearningResources,
    ContentCustomization,
    LearningGoals,
    QuestionGenerator,
}

impl App {
    pub fn new(api_key: String) -> Self {
        // Create a channel for communicating between the LLM task and the main app
        let (module_sender, module_receiver) = mpsc::channel(10);

        // Initialize config service
        let config_service = ConfigService::new();

        let llm_client = LlmClient::new(api_key.clone());

        Self {
            is_running: true,
            current_state:  AppState::Welcome,
            selected_level: 5, // Default level
            selected_index: IndexType::Random, // Default index
            index_selection_cursor: 0, // Default cursor position
            settings_cursor: 0, // Default settings cursor position
            settings_section: SettingsSection::LearningResources, // Default settings section
            show_help: false,
            show_quit_confirmation: false,
            quit_confirmation_selected: false, // Default to "No"
            llm_client: llm_client.clone(),
            api_key,
            scroll_offset: 0,
            current_module: None,
            popup_start_time: None,
            module_receiver,
            module_sender,
            config_service,
            // Initialize question generator fields
            question_set: None,
            generated_application: None,
            question_generator: Some(crate::question_generator::QuestionGenerator::new(llm_client)),
            question_generation_rx: None,
            application_generation_rx: None,
        }
    }

    pub fn tick(&mut self) {
        // Check if we're in the LevelTooLowPopup state and if the timer has expired
        if let AppState::LevelTooLowPopup = self.current_state {
            if let Some(start_time) = self.popup_start_time {
                // Check if 3 seconds have passed
                if start_time.elapsed().as_secs() >= 3 {
                    // Reset the timer
                    self.popup_start_time = None;
                    // Return to the welcome screen
                    self.current_state = AppState::Welcome;
                }
            }
        }

        // Check if we're in the QuestionGeneration state and poll for results
        if let AppState::QuestionGeneration = self.current_state {
            if let Some(ref mut rx) = self.question_generation_rx {
                match rx.try_recv() {
                    Ok(result) => {
                        match result {
                            Ok(question_set) => {
                                self.question_set = Some(question_set);
                                self.current_state = AppState::QuestionAnswering;
                            },
                            Err(err) => {
                                tracing::error!("Failed to generate questions: {}", err);
                                self.current_state = AppState::Learning;
                            }
                        }
                        self.question_generation_rx = None;
                    },
                    Err(mpsc::error::TryRecvError::Empty) => {
                        // Still waiting for the result
                    },
                    Err(mpsc::error::TryRecvError::Disconnected) => {
                        tracing::error!("Question generation task disconnected");
                        self.current_state = AppState::Learning;
                        self.question_generation_rx = None;
                    }
                }
            }
        }

        // Check if we're in the ApplicationGeneration state and poll for results
        if let AppState::ApplicationGeneration = self.current_state {
            if let Some(ref mut rx) = self.application_generation_rx {
                match rx.try_recv() {
                    Ok(result) => {
                        match result {
                            Ok(application) => {
                                self.generated_application = Some(application);
                                self.current_state = AppState::ApplicationDisplay;

                                // Transition to loading state
                                //self.current_state = AppState::Loading;

                                // Generate a learning module based on the selected level and index
                                self.generate_learning_module();

                            },
                            Err(err) => {
                                tracing::error!("Failed to generate application: {}", err);
                                self.current_state = AppState::QuestionAnswering;
                            }
                        }
                        self.application_generation_rx = None;
                    },
                    Err(mpsc::error::TryRecvError::Empty) => {
                        // Still waiting for the result
                    },
                    Err(mpsc::error::TryRecvError::Disconnected) => {
                        tracing::error!("Application generation task disconnected");
                        self.current_state = AppState::QuestionAnswering;
                        self.application_generation_rx = None;
                    }
                }
            }
        }

        // Check if we're in the Loading state and if there's a message from the LLM client
        if let AppState::Loading = self.current_state {
            // Try to receive a message from the channel (non-blocking)
            match self.module_receiver.try_recv() {
                Ok(result) => {
                    match result {
                        Ok(module) => {
                            // Generate additional resources if enabled
                            let mut module_with_resources = module.clone();
                            module_with_resources.additional_resources = self.generate_additional_resources(&module.topic);

                            // Update the state
                            self.current_module = Some(module_with_resources);
                            self.current_state = AppState::Learning;
                            self.scroll_offset = 0; // Reset scroll position for new content

                            // Create a Cargo project for the learning module
                            match cargo_project::create_cargo_project(&module, self.selected_level) {
                                Ok(project_dir) => {
                                    tracing::info!("Created Cargo project at: {:?}", project_dir);
                                }
                                Err(err) => {
                                    tracing::error!("Failed to create Cargo project: {}", err);
                                }
                            }
                        }
                        Err(err) => {
                            // There was an error generating the module
                            tracing::error!("Failed to generate learning module: {}", err);

                            // Create an error module
                            let error_module = LearningModule {
                                topic: "Error Generating Content".to_string(),
                                explanation: format!(
                                    "There was an error generating content: {}\n\nPlease try again or select a different level.",
                                    err
                                ),
                                code_snippets: vec![],
                                exercises: vec![],
                                additional_resources: None,
                            };

                            self.current_module = Some(error_module);
                            self.current_state = AppState::Learning;
                        }
                    }
                }
                Err(mpsc::error::TryRecvError::Empty) => {
                    // No message yet, continue waiting
                }
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    // Channel is disconnected, this shouldn't happen in normal operation
                    tracing::error!("Module channel disconnected");

                    // Create an error module
                    let error_module = LearningModule {
                        topic: "Communication Error".to_string(),
                        explanation: "There was an error communicating with the content generation service.\n\nPlease try again or select a different level.".to_string(),
                        code_snippets: vec![],
                        exercises: vec![],
                        additional_resources: None,
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
                }
                KeyCode::Left | KeyCode::Right | KeyCode::Char('h') | KeyCode::Char('l') => {
                    // Toggle between Yes and No
                    self.quit_confirmation_selected = !self.quit_confirmation_selected;
                }
                KeyCode::Esc | KeyCode::Char('q') => self.show_quit_confirmation = false,
                _ => {}
            }
            return Ok(());
        }

        match key_event.code {
            KeyCode::Char('q') => self.show_quit_confirmation = true,
            KeyCode::Char('?') => self.show_help = true,
            KeyCode::Char('s') => {
                // Toggle settings screen if not already in settings
                if self.current_state != AppState::Settings {
                    self.current_state = AppState::Settings;
                    self.settings_cursor = 0;
                    self.settings_section = SettingsSection::LearningResources;
                }
            },
            _ => {}
        }

        // Context-specific keybindings
        match self.current_state {
            AppState::Welcome => self.handle_welcome_keys(key_event),
            AppState::IndexSelection => self.handle_index_selection_keys(key_event),
            AppState::Learning => self.handle_learning_keys(key_event),
            AppState::Settings => self.handle_settings_keys(key_event),
            AppState::QuestionGeneration => {
                // No key handling in question generation state
                // This state is transient and will automatically transition to QuestionAnswering
            },
            AppState::QuestionAnswering => self.handle_question_answering_keys(key_event),
            AppState::ApplicationGeneration => {
                // No key handling in application generation state
                // This state is transient and will automatically transition to ApplicationDisplay
            },
            AppState::ApplicationDisplay => self.handle_application_display_keys(key_event),
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
                // Transition to index selection state
                self.current_state = AppState::IndexSelection;
                // Reset cursor position
                self.index_selection_cursor = 0;
            }
            _ => {}
        }
    }

    fn handle_index_selection_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Down | KeyCode::Char('j') => {
                // Move cursor down (0-2)
                self.index_selection_cursor = (self.index_selection_cursor + 1).min(3);
            },
            KeyCode::Up | KeyCode::Char('k') => {
                // Move cursor up
                self.index_selection_cursor = self.index_selection_cursor.saturating_sub(1);
            },
            KeyCode::Enter => {
                // Set the selected index based on cursor position
                self.selected_index = match self.index_selection_cursor {
                    0 => IndexType::RustLibrary,
                    1 => IndexType::RustByExample,
                    2 => IndexType::RustProgrammingLanguage,
                    3 => IndexType::Random,
                    _ => IndexType::Random,
                };

                // Check if the user selected the library index and if their level is less than 3
                if self.index_selection_cursor == 0 && self.selected_level < 3 {
                    // Show the level too low popup
                    self.current_state = AppState::LevelTooLowPopup;
                    // Start the timer
                    self.popup_start_time = Some(std::time::Instant::now());
                } else {
                    self.current_state = AppState::QuestionGeneration;
                    self.generate_questions();

                }
            },
            // KeyCode::Char('w') => {
            //     self.current_state = AppState::QuestionGeneration;
            //     self.generate_questions();
            // },
            KeyCode::Esc => {
                // Go back to welcome screen
                self.current_state = AppState::Welcome;
            }
            _ => {}
        }
    }

    fn generate_learning_module(&mut self) {
        // Get a random topic based on the user's level and selected index
        match data::get_random_topic_for_level(self.selected_level, &self.selected_index) {
            Ok(topic) => {
                // Clone the sender and topic for the async task
                let sender = self.module_sender.clone();
                let topic_clone = topic.clone();
                let level = self.selected_level;
                let llm_client = self.llm_client.clone();

                // Spawn an async task to call the LLM
                tokio::spawn(async move {
                    // Call the LLM to generate a learning module
                    let result = llm_client
                        .generate_learning_module(&topic_clone, level)
                        .await;

                    // Send the result back to the main thread
                    if let Err(e) = sender.send(result).await {
                        tracing::error!("Failed to send learning module: {}", e);
                    }
                });

                // The app remains in the Loading state until the async task completes
                // The tick method will handle the response when it arrives
            }
            Err(err) => {
                // If there was an error getting a topic, create an error module
                tracing::error!("Failed to get a random topic: {}", err);

                let module = LearningModule {
                    topic: "Error Loading Topic".to_string(),
                    explanation: format!(
                        "There was an error loading a topic for level {}. Please try again.",
                        self.selected_level
                    ),
                    code_snippets: vec![],
                    exercises: vec![],
                    additional_resources: None,
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
            KeyCode::Char('w') => {
                // Start the question generator
                self.current_state = AppState::QuestionGeneration;
                // Generate questions
                self.generate_questions();
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

    // Getter methods for config service
    pub fn get_learning_resources(&self) -> &crate::config::LearningResources {
        self.config_service.get_learning_resources()
    }

    pub fn get_content_customization(&self) -> &crate::config::ContentCustomization {
        self.config_service.get_content_customization()
    }

    pub fn get_question_generator_settings(&self) -> &crate::config::QuestionGeneratorSettings {
        self.config_service.get_question_generator_settings()
    }
    
    // Generate additional learning resources based on the topic
    pub fn generate_additional_resources(&self, topic: &str) -> Option<AdditionalResources> {
        // Only generate resources if we have a valid topic (not an error message)
        if topic.starts_with("Error") {
            return None;
        }

        let resources = self.config_service.get_learning_resources();

        // Skip if all resource types are disabled
        if !resources.show_official_docs && !resources.show_community_resources && 
           !resources.show_crates_io && !resources.show_github_repos {
            return None;
        }

        // Extract keywords from the topic
        let keywords: Vec<&str> = topic.split_whitespace()
            .filter(|word| word.len() > 3 && !["Rust", "rust", "The", "the", "and", "for"].contains(word))
            .collect();

        if keywords.is_empty() {
            return None;
        }

        let mut official_docs = Vec::new();
        let mut community_resources = Vec::new();
        let mut crates_io = Vec::new();
        let mut github_repos = Vec::new();

        // Add official documentation if enabled
        if resources.show_official_docs {
            // Rust standard library docs
            official_docs.push(Resource {
                title: "Rust Standard Library Documentation".to_string(),
                url: "https://doc.rust-lang.org/std/".to_string(),
                description: "Official documentation for the Rust standard library".to_string(),
            });

            // Rust Book
            official_docs.push(Resource {
                title: "The Rust Programming Language Book".to_string(),
                url: "https://doc.rust-lang.org/book/".to_string(),
                description: "Comprehensive guide to the Rust programming language".to_string(),
            });

            // Rust by Example
            official_docs.push(Resource {
                title: "Rust By Example".to_string(),
                url: "https://doc.rust-lang.org/rust-by-example/".to_string(),
                description: "Collection of runnable examples that illustrate various Rust concepts".to_string(),
            });

            // Add keyword-specific resources
            for keyword in &keywords {
                official_docs.push(Resource {
                    title: format!("Rust Documentation Search: {}", keyword),
                    url: format!("https://doc.rust-lang.org/std/?search={}", keyword),
                    description: format!("Search results for '{}' in the Rust documentation", keyword),
                });
            }
        }

        // Add community resources if enabled
        if resources.show_community_resources {
            // Rust Forum
            community_resources.push(Resource {
                title: "Rust Users Forum".to_string(),
                url: "https://users.rust-lang.org/".to_string(),
                description: "Official forum for Rust users to ask questions and share knowledge".to_string(),
            });

            // Rust Reddit
            community_resources.push(Resource {
                title: "Rust Subreddit".to_string(),
                url: "https://www.reddit.com/r/rust/".to_string(),
                description: "Reddit community for Rust developers".to_string(),
            });

            // Stack Overflow
            for keyword in &keywords {
                community_resources.push(Resource {
                    title: format!("Stack Overflow: Rust + {}", keyword),
                    url: format!("https://stackoverflow.com/questions/tagged/rust+{}", keyword),
                    description: format!("Stack Overflow questions about Rust and {}", keyword),
                });
            }
        }

        // Add crates.io links if enabled
        if resources.show_crates_io {
            // General crates.io link
            crates_io.push(Resource {
                title: "Crates.io - The Rust Package Registry".to_string(),
                url: "https://crates.io/".to_string(),
                description: "The official Rust package registry".to_string(),
            });

            // Keyword-specific crates
            for keyword in &keywords {
                crates_io.push(Resource {
                    title: format!("Crates.io Search: {}", keyword),
                    url: format!("https://crates.io/search?q={}", keyword),
                    description: format!("Rust packages related to {}", keyword),
                });
            }
        }

        // Add GitHub repositories if enabled
        if resources.show_github_repos {
            // Rust language repository
            github_repos.push(Resource {
                title: "Rust Language GitHub Repository".to_string(),
                url: "https://github.com/rust-lang/rust".to_string(),
                description: "The official Rust language repository".to_string(),
            });

            // Keyword-specific repositories
            for keyword in &keywords {
                github_repos.push(Resource {
                    title: format!("GitHub: Rust + {}", keyword),
                    url: format!("https://github.com/search?q=language:rust+{}", keyword),
                    description: format!("GitHub repositories related to Rust and {}", keyword),
                });
            }
        }

        Some(AdditionalResources {
            official_docs,
            community_resources,
            crates_io,
            github_repos,
        })
    }

    fn handle_settings_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                // Return to previous screen
                self.current_state = AppState::Welcome;
            }
            KeyCode::Tab => {
                // Cycle through settings sections
                self.settings_section = match self.settings_section {
                    SettingsSection::LearningResources => SettingsSection::ContentCustomization,
                    SettingsSection::ContentCustomization => SettingsSection::LearningGoals,
                    SettingsSection::LearningGoals => SettingsSection::QuestionGenerator,
                    SettingsSection::QuestionGenerator => SettingsSection::LearningResources,
                };
                self.settings_cursor = 0; // Reset cursor when changing sections
            }
            KeyCode::Up | KeyCode::Char('i') => {
                // Move cursor up
                if self.settings_cursor > 0 {
                    self.settings_cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('m') => {
                // Move cursor down
                let max_cursor = match self.settings_section {
                    SettingsSection::LearningResources => 3, // 4 options (0-3)
                    SettingsSection::ContentCustomization => 2, // 3 options (0-2)
                    SettingsSection::LearningGoals => 3, // 4 options (0-3)
                    SettingsSection::QuestionGenerator => 1, // Two options (0-1)
                };
                if self.settings_cursor < max_cursor {
                    self.settings_cursor += 1;
                }
            }
            KeyCode::Right | KeyCode::Char('k')  => {
                // Toggle or cycle the selected setting
                match self.settings_section {
                    SettingsSection::LearningResources => {
                        match self.settings_cursor {
                            0 => { let _ = self.config_service.toggle_official_docs(); }
                            1 => { let _ = self.config_service.toggle_community_resources(); }
                            2 => { let _ = self.config_service.toggle_crates_io(); }
                            3 => { let _ = self.config_service.toggle_github_repos(); }
                            _ => {}
                        }
                    }
                    SettingsSection::ContentCustomization => {
                        match self.settings_cursor {
                            0 => { let _ = self.config_service.cycle_code_complexity(); }
                            1 => { let _ = self.config_service.cycle_explanation_verbosity(); }
                            2 => { let _ = self.config_service.cycle_focus_area(); }
                            _ => {}
                        }

                    }
                    SettingsSection::LearningGoals => {
                        match self.settings_cursor {
                            0 => { let _ = self.config_service.cycle_learning_goal(); },
                            1 => { let _ = self.config_service.cycle_learning_goal(); },
                            2 => { let _ = self.config_service.cycle_learning_goal(); },
                            3 => { let _ = self.config_service.cycle_learning_goal(); },
                            _ => {}
                        }
                    },
                    SettingsSection::QuestionGenerator => {
                        match self.settings_cursor {
                            0 => { let _ = self.config_service.increment_num_questions(); },
                            1 => { let _ = self.config_service.cycle_question_type(); },
                            _ => {}
                        }
                    }
                }
            },
            KeyCode::Left | KeyCode::Char('j')  => {
                // Toggle or cycle the selected setting
                match self.settings_section {
                    SettingsSection::LearningResources => {
                        match self.settings_cursor {
                            0 => { let _ = self.config_service.toggle_official_docs(); }
                            1 => { let _ = self.config_service.toggle_community_resources(); }
                            2 => { let _ = self.config_service.toggle_crates_io(); }
                            3 => { let _ = self.config_service.toggle_github_repos(); }
                            _ => {}
                        }
                    }
                    SettingsSection::ContentCustomization => {
                        match self.settings_cursor {
                            0 => { let _ = self.config_service.cycle_code_complexity_reverse(); }
                            1 => { let _ = self.config_service.cycle_explanation_verbosity_reverse(); }
                            2 => { let _ = self.config_service.cycle_focus_area_reverse(); }
                            _ => {}
                        }

                    }
                    SettingsSection::LearningGoals => {
                        match self.settings_cursor {
                            0 => { let _ = self.config_service.cycle_learning_goal_reverse(); },
                            1 => { let _ = self.config_service.cycle_learning_goal_reverse(); },
                            2 => { let _ = self.config_service.cycle_learning_goal_reverse(); },
                            3 => { let _ = self.config_service.cycle_learning_goal_reverse(); },
                            _ => {}
                        }
                    },
                    SettingsSection::QuestionGenerator => {
                        match self.settings_cursor {
                            0 => { let _ = self.config_service.decrement_num_questions(); },
                            1 => { let _ = self.config_service.cycle_question_type(); },
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn get_learning_goal(&self) -> LearningGoal {
        self.config_service.get_content_customization().learning_goal
    }
    

    // Question generator methods
    pub fn generate_questions(&mut self) {
        let topic = match &self.current_module {
            Some(module) => module.topic.clone(),
            None => "Rust programming".to_string(),
        };

        let question_settings = self.get_question_generator_settings().clone();

        let learning_goal = self.get_learning_goal();
        let num_questions = question_settings.num_questions;
        let question_type= question_settings.default_question_type;
        // Generate questions using the question generator asynchronously
        if let Some(generator) = &self.question_generator {
            let generator_clone = generator.clone();
            let topic_clone = topic.clone();
            let learning_goal_clone = learning_goal.clone();
            
            // Spawn async task to generate questions
            let (tx, rx) = mpsc::unbounded_channel();
            
            tokio::spawn(async move {
                match generator_clone.generate_questions(&topic_clone, &learning_goal_clone, question_type, num_questions).await {
                    Ok(question_set) => {
                        let _ = tx.send(Ok(question_set));
                    },
                    Err(err) => {
                        let _ = tx.send(Err(err));
                    }
                }
            });
            
            // Store the receiver for polling in the update loop
            self.question_generation_rx = Some(rx);
        }
    }

    pub fn generate_application(&mut self) {
        if let (Some(generator), Some(question_set)) = (&self.question_generator, &self.question_set) {
            // Only generate an application if all questions have been answered
            if question_set.is_complete() {
                self.current_state = AppState::ApplicationGeneration;

                let generator_clone = generator.clone();
                let question_set_clone = question_set.clone();
                
                // Spawn async task to generate application
                let (tx, rx) = mpsc::unbounded_channel();
                
                tokio::spawn(async move {
                    match generator_clone.generate_application(&question_set_clone).await {
                        Ok(application) => {
                            let _ = tx.send(Ok(application));
                        },
                        Err(err) => {
                            let _ = tx.send(Err(err));
                        }
                    }
                });
                
                // Store the receiver for polling in the update loop
                self.application_generation_rx = Some(rx);
            }
        }
    }

    pub fn handle_question_answering_keys(&mut self, key_event: KeyEvent) {
        if let Some(question_set) = &mut self.question_set {
            match key_event.code {
                KeyCode::Esc => {
                    // Go back to the learning state
                    self.current_state = AppState::Learning;
                },
                KeyCode::Left | KeyCode::Char('h') => {
                    // Go to the previous question
                    question_set.previous_question();
                },
                KeyCode::Right | KeyCode::Char('l') => {
                    // Go to the next question
                    question_set.next_question();
                },
                KeyCode::Enter => {
                    // If all questions are answered, generate the application
                    if question_set.is_complete() {
                        self.generate_application();
                    }
                },
                KeyCode::Char(c) => {
                    // Handle answer selection
                    if let Some(current_question) = question_set.current_question_mut() {
                        match current_question.question_type {
                            crate::question_generator::QuestionType::Binary => {
                                // For binary questions, 'y' is Yes and 'n' is No
                                if c == 'y' || c == 'Y' {
                                    current_question.selected_answer = Some("Yes".to_string());
                                } else if c == 'n' || c == 'N' {
                                    current_question.selected_answer = Some("No".to_string());
                                }
                            },
                            crate::question_generator::QuestionType::Multiple => {
                                // For multiple choice, 1-4 or a-d are valid answers
                                if ('1'..='4').contains(&c) || ('a'..='d').contains(&c) {
                                    current_question.selected_answer = Some(c.to_string());
                                }
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }

    pub fn handle_application_display_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                // Go back to the learning state
                self.current_state = AppState::Learning;
            },
            KeyCode::Enter => {
                // Create a Cargo project from the generated application
                if let Some(application) = &self.generated_application {
                    match crate::cargo_project::create_application_project(application) {
                        Ok(project_dir) => {
                            tracing::info!("Created application project at: {:?}", project_dir);
                            // Go back to the learning state
                            self.current_state = AppState::Learning;
                        },
                        Err(err) => {
                            tracing::error!("Failed to create application project: {}", err);
                            // Stay in the current state
                        }
                    }
                }
            },
            _ => {}
        }
    }
}
