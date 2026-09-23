// src/app.rs
// Application state and input handling. Rendering lives in `ui`.
use crate::cargo_project;
use crate::config::{ConfigService, Cycle};
use crate::data::{self, Topic};
use crate::llm::{LlmClient, Model};
use crate::model::LearningModule;
use crate::question_generator::{GeneratedApplication, QuestionGenerator, QuestionRequest, QuestionSet};
use crate::resources;
use crate::settings::{SettingAction, SettingsSection};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use std::cell::Cell;
use std::future::Future;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

/// How long a status message stays visible
const STATUS_MESSAGE_DURATION: Duration = Duration::from_secs(5);
/// How many learning modules are kept for browsing back
const HISTORY_LIMIT: usize = 20;
/// How much streamed text is kept for the live preview
const PREVIEW_LIMIT: usize = 4000;

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl AppState {
    /// States that wait for a background LLM request
    pub fn is_busy(self) -> bool {
        matches!(self, AppState::Loading | AppState::QuestionGeneration | AppState::ApplicationGeneration)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IndexType {
    RustLibrary,
    RustByExample,
    RustProgrammingLanguage,
    Cyber,
    Random,
}

pub const INDEX_OPTIONS: [(IndexType, &str); 5] = [
    (IndexType::RustLibrary, "Rust Library Index (libraries like tokio, serde, etc.)"),
    (IndexType::RustByExample, "Rust By Example Index (examples from Rust By Example)"),
    (IndexType::RustProgrammingLanguage, "Rust Programming Language Index (topics from The Book)"),
    (IndexType::Cyber, "Cyber (security engineering: memory safety, crypto, analysis & hardening)"),
    (IndexType::Random, "Random (select randomly from the general Rust indexes)"),
];

/// Results of background tasks, delivered to the main loop over a single channel.
/// LLM results carry the id of the request that produced them, so results of
/// cancelled or superseded requests are ignored.
pub enum TaskResult {
    /// Text streamed in by a running request
    Progress {
        id: u64,
        text: String,
    },
    Module {
        id: u64,
        result: Result<LearningModule>,
    },
    Questions {
        id: u64,
        result: Result<QuestionSet>,
    },
    Application {
        id: u64,
        result: Result<GeneratedApplication>,
    },
    Models {
        id: u64,
        result: Result<Vec<Model>>,
    },
    ProjectCreated(Result<PathBuf>),
}

/// A running background request
struct PendingTask {
    id: u64,
    abort: AbortHandle,
}

/// Scroll position of a view. The limits are measured while rendering.
#[derive(Default)]
pub struct Scroll {
    pub offset: u16,
    pub max: Cell<u16>,
    pub page: Cell<u16>,
}

impl Scroll {
    pub fn reset(&mut self) {
        self.offset = 0;
    }

    /// Clamps the offset to the content measured by the last render
    pub fn clamped(&self) -> u16 {
        self.offset.min(self.max.get())
    }

    /// Handles scrolling keys; returns false if the key isn't a scrolling key
    fn handle_key(&mut self, code: KeyCode) -> bool {
        let page = self.page.get().saturating_sub(2).max(1);
        let offset = self.clamped();
        self.offset = match code {
            KeyCode::Up | KeyCode::Char('k') => offset.saturating_sub(1),
            KeyCode::Down | KeyCode::Char('j') => offset.saturating_add(1),
            KeyCode::PageUp | KeyCode::Char('b') => offset.saturating_sub(page),
            KeyCode::PageDown | KeyCode::Char(' ') => offset.saturating_add(page),
            KeyCode::Home | KeyCode::Char('g') => 0,
            KeyCode::End | KeyCode::Char('G') => self.max.get(),
            _ => return false,
        }
        .min(self.max.get());
        true
    }
}

/// Outcome of a key press in a filterable list popup
enum PickerKey {
    Handled,
    Cancel,
    Select,
}

/// Shared key handling of the filterable list popups: typing filters,
/// arrows and PgUp/PgDn move, Enter selects, Esc cancels
fn handle_picker_key(code: KeyCode, filter: &mut String, cursor: &mut usize, count: usize) -> PickerKey {
    let last = count.saturating_sub(1);
    match code {
        KeyCode::Esc => return PickerKey::Cancel,
        KeyCode::Enter => return PickerKey::Select,
        KeyCode::Up => *cursor = cursor.saturating_sub(1),
        KeyCode::Down => *cursor = (*cursor + 1).min(last),
        KeyCode::PageUp => *cursor = cursor.saturating_sub(10),
        KeyCode::PageDown => *cursor = (*cursor + 10).min(last),
        KeyCode::Home => *cursor = 0,
        KeyCode::End => *cursor = last,
        KeyCode::Backspace => {
            filter.pop();
            *cursor = 0;
        }
        KeyCode::Char(c) => {
            filter.push(c);
            *cursor = 0;
        }
        _ => {}
    }
    PickerKey::Handled
}

/// What happens after a topic is chosen
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TopicPurpose {
    /// Chosen after selecting a content source: ask the questions next
    StartQuestions,
    /// Chosen on the learning screen: generate a module about it
    NextModule,
}

/// An entry of the topic picker
#[derive(Clone, Debug, PartialEq)]
pub enum TopicChoice {
    Random,
    Custom(String),
    Listed(Topic),
}

/// The topic picker popup: the topics of the selected source for the user's level,
/// plus a random topic and a custom topic typed by the user
pub struct TopicPicker {
    pub topics: Vec<Topic>,
    pub filter: String,
    pub cursor: usize,
    pub purpose: TopicPurpose,
}

impl TopicPicker {
    /// Entries matching the filter: "Random" (without a filter), the matching
    /// topics, and the filter text as a custom topic
    pub fn choices(&self) -> Vec<TopicChoice> {
        let filter = self.filter.trim();
        let needle = filter.to_lowercase();
        let mut choices = Vec::new();
        if filter.is_empty() {
            choices.push(TopicChoice::Random);
        }
        choices.extend(
            self.topics
                .iter()
                .filter(|t| {
                    needle.is_empty()
                        || t.topic.to_lowercase().contains(&needle)
                        || t.source.to_lowercase().contains(&needle)
                })
                .cloned()
                .map(TopicChoice::Listed),
        );
        if !filter.is_empty() {
            choices.push(TopicChoice::Custom(filter.to_string()));
        }
        choices
    }
}

/// The model picker popup opened from the settings
pub struct ModelPicker {
    pub models: Option<Vec<Model>>, // None while loading
    pub filter: String,
    pub cursor: usize,
}

impl ModelPicker {
    /// Models matching the filter
    pub fn filtered(&self) -> Vec<&Model> {
        let filter = self.filter.to_lowercase();
        self.models
            .iter()
            .flatten()
            .filter(|m| {
                filter.is_empty() || m.id.to_lowercase().contains(&filter) || m.name.to_lowercase().contains(&filter)
            })
            .collect()
    }
}

pub struct App {
    pub is_running: bool,
    pub current_state: AppState,
    pub selected_level: u8,
    pub selected_index: IndexType,
    pub index_selection_cursor: usize,
    pub settings_cursor: usize,
    pub settings_section: SettingsSection,
    pub show_help: bool,
    pub show_quit_confirmation: bool,
    pub quit_confirmation_selected: bool, // true = Yes, false = No
    pub learning_scroll: Scroll,
    pub application_scroll: Scroll,
    pub help_scroll: Scroll,
    pub current_module: Option<LearningModule>,
    pub module_version: u64, // Changes whenever `current_module` changes (for render caching)
    pub history: Vec<LearningModule>,
    pub history_index: usize,
    pub popup_start_time: Option<Instant>,
    pub last_error: Option<String>, // Shown in an error popup until dismissed
    pub status_message: Option<(String, Instant)>, // Short-lived info message
    pub stream_preview: String,     // Text streamed in by the request the user is waiting for
    pub stream_chars: usize,
    pub model_picker: Option<ModelPicker>,
    pub topic_picker: Option<TopicPicker>,
    tick_count: u64,
    current_topic: Option<Topic>, // Topic shared by the questions and the learning module
    settings_return_state: AppState,
    question_return_state: AppState,
    llm_client: LlmClient,
    config_service: ConfigService,
    task_sender: mpsc::UnboundedSender<TaskResult>,
    next_task_id: u64,
    pending_module: Option<PendingTask>,
    pending_questions: Option<PendingTask>,
    pending_application: Option<PendingTask>,
    pending_models: Option<PendingTask>,
    pub question_set: Option<QuestionSet>,
    pub generated_application: Option<GeneratedApplication>,
    pub application_version: u64,
    question_generator: QuestionGenerator,
}

impl App {
    /// Creates the app and the receiver on which background task results arrive
    pub fn new(api_key: String, config_service: ConfigService) -> (Self, mpsc::UnboundedReceiver<TaskResult>) {
        let (task_sender, task_receiver) = mpsc::unbounded_channel();
        let llm_client = LlmClient::new(api_key);

        let app = Self {
            is_running: true,
            current_state: AppState::Welcome,
            selected_level: 5,
            selected_index: IndexType::Random,
            index_selection_cursor: 0,
            settings_cursor: 0,
            settings_section: SettingsSection::LearningResources,
            show_help: false,
            show_quit_confirmation: false,
            quit_confirmation_selected: false,
            learning_scroll: Scroll::default(),
            application_scroll: Scroll::default(),
            help_scroll: Scroll::default(),
            current_module: None,
            module_version: 0,
            history: Vec::new(),
            history_index: 0,
            popup_start_time: None,
            last_error: None,
            status_message: None,
            stream_preview: String::new(),
            stream_chars: 0,
            model_picker: None,
            topic_picker: None,
            tick_count: 0,
            current_topic: None,
            settings_return_state: AppState::Welcome,
            question_return_state: AppState::IndexSelection,
            question_generator: QuestionGenerator::new(llm_client.clone()),
            llm_client,
            config_service,
            task_sender,
            next_task_id: 0,
            pending_module: None,
            pending_questions: None,
            pending_application: None,
            pending_models: None,
            question_set: None,
            generated_application: None,
            application_version: 0,
        };
        (app, task_receiver)
    }

    pub fn config(&self) -> &crate::config::Config {
        self.config_service.config()
    }

    pub fn config_path(&self) -> Option<&std::path::Path> {
        self.config_service.path()
    }

    pub fn tick(&mut self) {
        self.tick_count = self.tick_count.wrapping_add(1);

        // Leave the "level too low" popup after 3 seconds
        if self.current_state == AppState::LevelTooLowPopup
            && self.popup_start_time.is_some_and(|start| start.elapsed() >= Duration::from_secs(3))
        {
            self.popup_start_time = None;
            self.current_state = AppState::Welcome;
        }

        // Expire the status message
        if self.status_message.as_ref().is_some_and(|(_, shown_at)| shown_at.elapsed() >= STATUS_MESSAGE_DURATION) {
            self.status_message = None;
        }
    }

    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    /// Whether a learning module is being generated in the background
    pub fn is_module_pending(&self) -> bool {
        self.pending_module.is_some()
    }

    fn show_error(&mut self, message: String) {
        tracing::error!("{}", message);
        self.last_error = Some(message);
    }

    fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some((message.into(), Instant::now()));
    }

    fn reset_stream_preview(&mut self) {
        self.stream_preview.clear();
        self.stream_chars = 0;
    }

    fn next_id(&mut self) -> u64 {
        self.next_task_id += 1;
        self.next_task_id
    }

    /// Spawns a background request. `work` receives a callback for streamed text.
    /// The result is sent back over the task channel tagged with the request id;
    /// panics are reported as errors.
    fn spawn_request<T, F, Fut>(&mut self, work: F, wrap: fn(u64, Result<T>) -> TaskResult) -> PendingTask
    where
        T: Send + 'static,
        F: FnOnce(Box<dyn FnMut(&str) + Send>) -> Fut,
        Fut: Future<Output = Result<T>> + Send + 'static,
    {
        let id = self.next_id();
        let progress_sender = self.task_sender.clone();
        let on_text: Box<dyn FnMut(&str) + Send> = Box::new(move |text: &str| {
            let _ = progress_sender.send(TaskResult::Progress { id, text: text.to_string() });
        });

        let handle = tokio::spawn(work(on_text));
        let abort = handle.abort_handle();
        let sender = self.task_sender.clone();
        tokio::spawn(async move {
            let result = match handle.await {
                Ok(result) => result,
                Err(err) if err.is_cancelled() => return,
                Err(err) => Err(anyhow::anyhow!("Background task failed: {}", err)),
            };
            let _ = sender.send(wrap(id, result));
        });

        PendingTask { id, abort }
    }

    fn cancel(task: &mut Option<PendingTask>) {
        if let Some(task) = task.take() {
            task.abort.abort();
        }
    }

    /// Clears `slot` and returns true if it holds the request with this id
    fn take_if_current(slot: &mut Option<PendingTask>, id: u64) -> bool {
        if slot.as_ref().is_some_and(|task| task.id == id) {
            *slot = None;
            true
        } else {
            false
        }
    }

    /// Whether `id` is a running content request (whose text feeds the live preview)
    fn is_content_request(&self, id: u64) -> bool {
        [&self.pending_module, &self.pending_questions, &self.pending_application]
            .iter()
            .any(|task| task.as_ref().is_some_and(|t| t.id == id))
    }

    pub fn handle_task_result(&mut self, task_result: TaskResult) {
        match task_result {
            TaskResult::Progress { id, text } => {
                // Also collected while the module is generated in the background,
                // so the preview is complete when the user starts waiting for it
                if self.is_content_request(id) {
                    self.stream_chars += text.chars().count();
                    self.stream_preview.push_str(&text);
                    if self.stream_preview.len() > PREVIEW_LIMIT {
                        let mut cut = self.stream_preview.len() - PREVIEW_LIMIT / 2;
                        while !self.stream_preview.is_char_boundary(cut) {
                            cut += 1;
                        }
                        self.stream_preview.drain(..cut);
                    }
                }
            }
            TaskResult::Module { id, result } => {
                if !Self::take_if_current(&mut self.pending_module, id) {
                    return; // Stale or cancelled request
                }
                match result {
                    Ok(mut module) => {
                        module.additional_resources =
                            resources::for_topic(&module.topic, &self.config().learning_resources);
                        self.add_to_history(module);
                    }
                    Err(err) => self.show_error(format!("Failed to generate learning module: {:#}", err)),
                }
                if self.current_state == AppState::Loading {
                    self.current_state = AppState::Learning;
                }
            }
            TaskResult::Questions { id, result } => {
                if !Self::take_if_current(&mut self.pending_questions, id) {
                    return;
                }
                match result {
                    Ok(question_set) => {
                        self.question_set = Some(question_set);
                        if self.current_state == AppState::QuestionGeneration {
                            self.current_state = AppState::QuestionAnswering;
                        }
                    }
                    Err(err) => {
                        self.show_error(format!("Failed to generate questions: {:#}", err));
                        if self.current_state == AppState::QuestionGeneration {
                            self.current_state = self.question_return_state;
                        }
                    }
                }
            }
            TaskResult::Application { id, result } => {
                if !Self::take_if_current(&mut self.pending_application, id) {
                    return;
                }
                match result {
                    Ok(application) => {
                        self.generated_application = Some(application);
                        self.application_version += 1;
                        self.application_scroll.reset();
                        if self.current_state == AppState::ApplicationGeneration {
                            self.current_state = AppState::ApplicationDisplay;
                        }
                        // Prepare the learning module for the same topic while the
                        // user looks at the generated application
                        self.generate_learning_module(false);
                    }
                    Err(err) => {
                        self.show_error(format!("Failed to generate application: {:#}", err));
                        if self.current_state == AppState::ApplicationGeneration {
                            self.current_state = AppState::QuestionAnswering;
                        }
                    }
                }
            }
            TaskResult::Models { id, result } => {
                if !Self::take_if_current(&mut self.pending_models, id) {
                    return;
                }
                match result {
                    Ok(models) => {
                        if let Some(picker) = &mut self.model_picker {
                            let current = &self.config_service.config().model;
                            picker.cursor = models.iter().position(|m| &m.id == current).unwrap_or(0);
                            picker.models = Some(models);
                        }
                    }
                    Err(err) => {
                        self.model_picker = None;
                        self.show_error(format!("Failed to load the model list: {:#}", err));
                    }
                }
            }
            TaskResult::ProjectCreated(result) => match result {
                Ok(project_dir) => {
                    tracing::info!("Created Cargo project at: {:?}", project_dir);
                    self.set_status(format!("Created Cargo project at {}", project_dir.display()));
                }
                Err(err) => self.show_error(format!("Failed to create Cargo project: {:#}", err)),
            },
        }
    }

    fn add_to_history(&mut self, module: LearningModule) {
        self.history.push(module);
        if self.history.len() > HISTORY_LIMIT {
            self.history.remove(0);
        }
        self.show_history_entry(self.history.len() - 1);
    }

    fn show_history_entry(&mut self, index: usize) {
        if let Some(module) = self.history.get(index) {
            self.history_index = index;
            self.current_module = Some(module.clone());
            self.module_version += 1;
            self.learning_scroll.reset();
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        // Ignore key release/repeat events (reported on some platforms)
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }

        // An error popup swallows the next key press
        if self.last_error.is_some() {
            self.last_error = None;
            return Ok(());
        }

        if self.show_help {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => self.show_help = false,
                code => {
                    self.help_scroll.handle_key(code);
                }
            }
            return Ok(());
        }

        if self.show_quit_confirmation {
            match key_event.code {
                KeyCode::Enter => {
                    if self.quit_confirmation_selected {
                        self.is_running = false;
                    } else {
                        self.show_quit_confirmation = false;
                    }
                }
                KeyCode::Left | KeyCode::Right | KeyCode::Char('h') | KeyCode::Char('l') | KeyCode::Tab => {
                    self.quit_confirmation_selected = !self.quit_confirmation_selected;
                }
                KeyCode::Char('y') => self.is_running = false,
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('n') => self.show_quit_confirmation = false,
                _ => {}
            }
            return Ok(());
        }

        // The model picker takes all keys (typing filters the list)
        if self.model_picker.is_some() {
            self.handle_model_picker_keys(key_event);
            return Ok(());
        }
        if self.topic_picker.is_some() {
            self.handle_topic_picker_keys(key_event);
            return Ok(());
        }

        match key_event.code {
            KeyCode::Char('q') => {
                self.quit_confirmation_selected = false;
                self.show_quit_confirmation = true;
                return Ok(());
            }
            KeyCode::Char('?') => {
                self.help_scroll.reset();
                self.show_help = true;
                return Ok(());
            }
            KeyCode::Char('s')
                if self.current_state != AppState::Settings
                    && self.current_state != AppState::LevelTooLowPopup
                    && !self.current_state.is_busy() =>
            {
                self.settings_return_state = self.current_state;
                self.current_state = AppState::Settings;
                self.settings_cursor = 0;
                self.settings_section = SettingsSection::LearningResources;
                return Ok(());
            }
            _ => {}
        }

        match self.current_state {
            AppState::Welcome => self.handle_welcome_keys(key_event),
            AppState::IndexSelection => self.handle_index_selection_keys(key_event),
            AppState::Learning => self.handle_learning_keys(key_event),
            AppState::Settings => self.handle_settings_keys(key_event),
            AppState::Loading | AppState::QuestionGeneration | AppState::ApplicationGeneration => {
                if key_event.code == KeyCode::Esc {
                    self.cancel_current_request();
                }
            }
            AppState::QuestionAnswering => self.handle_question_answering_keys(key_event),
            AppState::ApplicationDisplay => self.handle_application_display_keys(key_event),
            AppState::LevelTooLowPopup => {}
        }
        Ok(())
    }

    /// Cancels the request the current screen is waiting for
    fn cancel_current_request(&mut self) {
        match self.current_state {
            AppState::Loading => {
                Self::cancel(&mut self.pending_module);
                self.current_state = AppState::Learning;
            }
            AppState::QuestionGeneration => {
                Self::cancel(&mut self.pending_questions);
                self.current_state = self.question_return_state;
            }
            AppState::ApplicationGeneration => {
                Self::cancel(&mut self.pending_application);
                self.current_state = AppState::QuestionAnswering;
            }
            _ => {}
        }
        self.set_status("Request cancelled");
    }

    fn handle_welcome_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Down | KeyCode::Char('j') => self.selected_level = (self.selected_level + 1).min(10),
            KeyCode::Up | KeyCode::Char('k') => self.selected_level = self.selected_level.saturating_sub(1).max(1),
            KeyCode::Char(c @ '1'..='9') => self.selected_level = c as u8 - b'0',
            KeyCode::Char('0') => self.selected_level = 10,
            KeyCode::Enter => {
                self.current_state = AppState::IndexSelection;
                self.index_selection_cursor = 0;
            }
            _ => {}
        }
    }

    fn handle_index_selection_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Down | KeyCode::Char('j') => {
                self.index_selection_cursor = (self.index_selection_cursor + 1).min(INDEX_OPTIONS.len() - 1);
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.index_selection_cursor = self.index_selection_cursor.saturating_sub(1);
            }
            KeyCode::Enter => {
                self.selected_index = INDEX_OPTIONS[self.index_selection_cursor].0;

                // Library topics need at least level 3
                if self.selected_index == IndexType::RustLibrary && self.selected_level < 3 {
                    self.current_state = AppState::LevelTooLowPopup;
                    self.popup_start_time = Some(Instant::now());
                    return;
                }

                // The questions, the application and the learning module are all
                // about the topic chosen next
                self.open_topic_picker(TopicPurpose::StartQuestions);
            }
            KeyCode::Esc => self.current_state = AppState::Welcome,
            _ => {}
        }
    }

    /// Opens the topic picker with the topics of the selected source for the user's level
    fn open_topic_picker(&mut self, purpose: TopicPurpose) {
        match data::topics_for_level(self.selected_level, &self.selected_index) {
            Ok(topics) => {
                self.topic_picker = Some(TopicPicker { topics, filter: String::new(), cursor: 0, purpose });
            }
            Err(err) => self.show_error(format!("Could not load the topics: {:#}", err)),
        }
    }

    fn handle_topic_picker_keys(&mut self, key_event: KeyEvent) {
        let Some(picker) = &mut self.topic_picker else {
            return;
        };
        let count = picker.choices().len();
        match handle_picker_key(key_event.code, &mut picker.filter, &mut picker.cursor, count) {
            PickerKey::Handled => {}
            PickerKey::Cancel => self.topic_picker = None,
            PickerKey::Select => {
                let choice = picker.choices().into_iter().nth(picker.cursor);
                let purpose = picker.purpose;
                self.topic_picker = None;
                if let Some(choice) = choice {
                    self.choose_topic(choice, purpose);
                }
            }
        }
    }

    /// Makes `choice` the current topic and continues with questions or a learning module
    fn choose_topic(&mut self, choice: TopicChoice, purpose: TopicPurpose) {
        match choice {
            TopicChoice::Random => {
                if !self.pick_new_topic() {
                    return;
                }
            }
            TopicChoice::Custom(text) => {
                self.current_topic = Some(Topic {
                    topic: text,
                    source: "Custom topic requested by the learner".to_string(),
                    min_level: self.selected_level,
                });
            }
            TopicChoice::Listed(topic) => self.current_topic = Some(topic),
        }

        match purpose {
            TopicPurpose::StartQuestions => {
                self.question_set = None;
                self.generated_application = None;
                Self::cancel(&mut self.pending_module);
                Self::cancel(&mut self.pending_application);
                self.question_return_state = AppState::IndexSelection;
                self.generate_questions();
            }
            TopicPurpose::NextModule => {
                self.current_state = AppState::Loading;
                self.generate_learning_module(false);
            }
        }
    }

    /// Picks a random topic for the selected level and index.
    /// Shows an error and returns false if none could be found.
    fn pick_new_topic(&mut self) -> bool {
        match data::get_random_topic_for_level(self.selected_level, &self.selected_index) {
            Ok(topic) => {
                self.current_topic = Some(topic);
                true
            }
            Err(err) => {
                self.show_error(format!("Could not find a topic for level {}: {:#}", self.selected_level, err));
                false
            }
        }
    }

    /// Starts generating a learning module in the background, for a new topic
    /// or for the current one. The result is handled in `handle_task_result`.
    fn generate_learning_module(&mut self, new_topic: bool) {
        if (new_topic || self.current_topic.is_none()) && !self.pick_new_topic() {
            if self.current_state == AppState::Loading {
                self.current_state = AppState::Learning;
            }
            return;
        }
        let Some(topic) = self.current_topic.clone() else {
            return;
        };

        Self::cancel(&mut self.pending_module);
        self.reset_stream_preview();

        let level = self.selected_level;
        let llm_client = self.llm_client.clone();
        let config = self.config().clone();
        let task = self.spawn_request(
            move |on_text| async move { llm_client.generate_learning_module(&topic, level, &config, on_text).await },
            |id, result| TaskResult::Module { id, result },
        );
        self.pending_module = Some(task);
    }

    fn handle_learning_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('n') => {
                self.current_state = AppState::Loading;
                self.generate_learning_module(true);
            }
            KeyCode::Char('t') => self.open_topic_picker(TopicPurpose::NextModule),
            KeyCode::Char('w') => {
                self.question_return_state = AppState::Learning;
                self.generate_questions();
            }
            KeyCode::Char('c') => {
                if let Some(module) = self.current_module.clone() {
                    let level = self.selected_level;
                    let base_dir = self.config().projects_dir();
                    self.create_project_in_background(move || {
                        cargo_project::create_cargo_project(&base_dir, &module, level)
                    });
                }
            }
            KeyCode::Char('[') => {
                if self.history_index > 0 {
                    self.show_history_entry(self.history_index - 1);
                }
            }
            KeyCode::Char(']') => {
                if self.history_index + 1 < self.history.len() {
                    self.show_history_entry(self.history_index + 1);
                }
            }
            KeyCode::Esc => self.current_state = AppState::Welcome,
            code => {
                self.learning_scroll.handle_key(code);
            }
        }
    }

    /// Runs Cargo project creation off the UI thread
    fn create_project_in_background<F>(&mut self, create: F)
    where
        F: FnOnce() -> Result<PathBuf> + Send + 'static,
    {
        let sender = self.task_sender.clone();
        tokio::task::spawn_blocking(move || {
            let _ = sender.send(TaskResult::ProjectCreated(create()));
        });
        self.set_status("Creating Cargo project...");
    }

    fn handle_settings_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.current_state = self.settings_return_state,
            KeyCode::Tab => {
                self.settings_section = self.settings_section.cycle(true);
                self.settings_cursor = 0;
            }
            KeyCode::BackTab => {
                self.settings_section = self.settings_section.cycle(false);
                self.settings_cursor = 0;
            }
            KeyCode::Up | KeyCode::Char('k') => self.settings_cursor = self.settings_cursor.saturating_sub(1),
            KeyCode::Down | KeyCode::Char('j') => {
                let max_cursor = self.settings_section.items().len().saturating_sub(1);
                self.settings_cursor = (self.settings_cursor + 1).min(max_cursor);
            }
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Enter | KeyCode::Char(' ') => {
                self.change_selected_setting(true);
            }
            KeyCode::Left | KeyCode::Char('h') => self.change_selected_setting(false),
            _ => {}
        }
    }

    /// Changes the setting under the cursor, forward or backward
    fn change_selected_setting(&mut self, forward: bool) {
        let Some(item) = self.settings_section.items().get(self.settings_cursor) else {
            return;
        };
        match item.action {
            SettingAction::Change(change) => {
                if let Err(err) = self.config_service.update(|config| change(config, forward)) {
                    self.show_error(format!("Failed to save settings: {:#}", err));
                }
            }
            SettingAction::PickModel => self.open_model_picker(),
        }
    }

    fn open_model_picker(&mut self) {
        self.model_picker = Some(ModelPicker { models: None, filter: String::new(), cursor: 0 });
        let llm_client = self.llm_client.clone();
        Self::cancel(&mut self.pending_models);
        let task = self.spawn_request(
            move |_| async move { llm_client.list_models().await },
            |id, result| TaskResult::Models { id, result },
        );
        self.pending_models = Some(task);
    }

    fn handle_model_picker_keys(&mut self, key_event: KeyEvent) {
        let Some(picker) = &mut self.model_picker else {
            return;
        };
        let count = picker.filtered().len();
        match handle_picker_key(key_event.code, &mut picker.filter, &mut picker.cursor, count) {
            PickerKey::Handled => {}
            PickerKey::Cancel => {
                self.model_picker = None;
                Self::cancel(&mut self.pending_models);
            }
            PickerKey::Select => {
                let Some(model) = picker.filtered().get(picker.cursor).map(|m| m.id.clone()) else {
                    return;
                };
                self.model_picker = None;
                match self.config_service.update(|config| config.model = model.clone()) {
                    Ok(()) => self.set_status(format!("Model set to {}", model)),
                    Err(err) => self.show_error(format!("Failed to save settings: {:#}", err)),
                }
            }
        }
    }

    pub fn generate_questions(&mut self) {
        let topic = match &self.current_topic {
            Some(topic) => topic.topic.clone(),
            None => "Rust programming".to_string(),
        };

        let config = self.config();
        let request = QuestionRequest {
            model: config.model.clone(),
            topic,
            level: self.selected_level,
            learning_goal: config.content_customization.learning_goal,
            style: config.question_generator_settings.question_style,
            num_questions: config.question_generator_settings.num_questions,
        };
        let generator = self.question_generator.clone();

        Self::cancel(&mut self.pending_questions);
        self.reset_stream_preview();
        let task = self.spawn_request(
            move |on_text| async move { generator.generate_questions(request, on_text).await },
            |id, result| TaskResult::Questions { id, result },
        );
        self.pending_questions = Some(task);
        self.current_state = AppState::QuestionGeneration;
    }

    pub fn generate_application(&mut self) {
        let Some(question_set) = self.question_set.clone() else {
            return;
        };
        if !question_set.is_complete() {
            return;
        }

        let generator = self.question_generator.clone();
        let model = self.config().model.clone();
        let level = self.selected_level;

        Self::cancel(&mut self.pending_application);
        self.reset_stream_preview();
        let task = self.spawn_request(
            move |on_text| async move { generator.generate_application(&model, &question_set, level, on_text).await },
            |id, result| TaskResult::Application { id, result },
        );
        self.pending_application = Some(task);
        self.current_state = AppState::ApplicationGeneration;
    }

    pub fn handle_question_answering_keys(&mut self, key_event: KeyEvent) {
        let Some(question_set) = &mut self.question_set else {
            if key_event.code == KeyCode::Esc {
                self.current_state = self.question_return_state;
            }
            return;
        };
        match key_event.code {
            KeyCode::Esc => self.current_state = self.question_return_state,
            KeyCode::Left | KeyCode::Char('h') => question_set.previous_question(),
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Tab => question_set.next_question(),
            KeyCode::Enter => {
                if question_set.is_complete() {
                    if self.config().question_generator_settings.enable_application_generation {
                        self.generate_application();
                    } else {
                        self.current_state = AppState::Loading;
                        self.generate_learning_module(false);
                    }
                } else {
                    let (answered, total) = question_set.progress();
                    self.set_status(format!("Answer all questions first ({}/{} answered)", answered, total));
                }
            }
            KeyCode::Char(c) => {
                if let Some(question) = question_set.current_question_mut()
                    && question.answer(c)
                {
                    question_set.advance_to_unanswered();
                }
            }
            _ => {}
        }
    }

    pub fn handle_application_display_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.leave_application_display(),
            KeyCode::Enter | KeyCode::Char('c') => {
                if let Some(application) = self.generated_application.clone() {
                    let base_dir = self.config().projects_dir();
                    self.create_project_in_background(move || {
                        cargo_project::create_application_project(&base_dir, &application)
                    });
                    self.leave_application_display();
                }
            }
            code => {
                self.application_scroll.handle_key(code);
            }
        }
    }

    /// Continues to the learning module, waiting for it if it is still being generated
    fn leave_application_display(&mut self) {
        self.current_state = if self.is_module_pending() { AppState::Loading } else { AppState::Learning };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::question_generator::{AnswerOption, Question, QuestionType};
    use crossterm::event::KeyModifiers;

    fn test_app() -> (App, mpsc::UnboundedReceiver<TaskResult>) {
        // Empty API key: requests fail immediately without touching the network
        App::new(String::new(), ConfigService::in_memory(Config::default()))
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn press(app: &mut App, code: KeyCode) {
        app.handle_key_event(key(code)).unwrap();
    }

    fn module(topic: &str) -> LearningModule {
        LearningModule {
            topic: topic.to_string(),
            explanation: String::new(),
            code_snippets: vec![],
            exercises: vec![],
            additional_resources: None,
        }
    }

    fn application() -> GeneratedApplication {
        GeneratedApplication {
            name: "App".to_string(),
            description: String::new(),
            features: vec![],
            code_snippets: vec![],
        }
    }

    fn question(question_type: QuestionType) -> Question {
        Question {
            id: 0,
            text: "Q".to_string(),
            question_type,
            options: (1..=4).map(|i| AnswerOption { id: i.to_string(), text: format!("Option {}", i) }).collect(),
            selected_answer: None,
        }
    }

    fn answered_question_set() -> QuestionSet {
        let mut q = question(QuestionType::Binary);
        q.selected_answer = Some("Yes".to_string());
        QuestionSet::new("topic".to_string(), vec![q])
    }

    #[tokio::test]
    async fn stale_module_result_is_ignored() {
        let (mut app, _rx) = test_app();
        app.current_state = AppState::Learning;

        press(&mut app, KeyCode::Char('n'));
        let first_id = app.pending_module.as_ref().unwrap().id;
        press(&mut app, KeyCode::Esc); // cancel
        assert_eq!(app.current_state, AppState::Learning);

        press(&mut app, KeyCode::Char('n'));
        let second_id = app.pending_module.as_ref().unwrap().id;
        assert_ne!(first_id, second_id);

        // The cancelled request's result must not replace anything
        app.handle_task_result(TaskResult::Module { id: first_id, result: Ok(module("old")) });
        assert!(app.current_module.is_none());
        assert_eq!(app.current_state, AppState::Loading);

        app.handle_task_result(TaskResult::Module { id: second_id, result: Ok(module("new")) });
        assert_eq!(app.current_module.as_ref().unwrap().topic, "new");
        assert_eq!(app.current_state, AppState::Learning);
    }

    #[tokio::test]
    async fn progress_is_collected_only_for_running_requests() {
        let (mut app, _rx) = test_app();
        app.current_state = AppState::Learning;
        press(&mut app, KeyCode::Char('n'));
        let id = app.pending_module.as_ref().unwrap().id;

        app.handle_task_result(TaskResult::Progress { id: id + 100, text: "stale".to_string() });
        assert!(app.stream_preview.is_empty());
        app.handle_task_result(TaskResult::Progress { id, text: "Hello ".to_string() });
        app.handle_task_result(TaskResult::Progress { id, text: "world".to_string() });
        assert_eq!(app.stream_preview, "Hello world");
        assert_eq!(app.stream_chars, 11);

        // The preview stays bounded
        for _ in 0..1000 {
            app.handle_task_result(TaskResult::Progress { id, text: "0123456789".to_string() });
        }
        assert!(app.stream_preview.len() <= PREVIEW_LIMIT);
    }

    #[tokio::test]
    async fn application_flow_waits_for_module() {
        let (mut app, _rx) = test_app();
        app.question_set = Some(answered_question_set());
        app.current_state = AppState::QuestionAnswering;

        press(&mut app, KeyCode::Enter);
        assert_eq!(app.current_state, AppState::ApplicationGeneration);
        let app_id = app.pending_application.as_ref().unwrap().id;

        app.handle_task_result(TaskResult::Application { id: app_id, result: Ok(application()) });
        assert_eq!(app.current_state, AppState::ApplicationDisplay);
        assert!(app.is_module_pending(), "module generation starts in the background");

        // Leaving the application screen waits for the module
        press(&mut app, KeyCode::Esc);
        assert_eq!(app.current_state, AppState::Loading);

        let module_id = app.pending_module.as_ref().unwrap().id;
        app.handle_task_result(TaskResult::Module { id: module_id, result: Ok(module("topic")) });
        assert_eq!(app.current_state, AppState::Learning);
        assert!(app.current_module.is_some());
    }

    #[tokio::test]
    async fn application_generation_can_be_disabled() {
        let (mut app, _rx) = test_app();
        app.config_service.update(|c| c.question_generator_settings.enable_application_generation = false).unwrap();
        app.question_set = Some(answered_question_set());
        app.current_state = AppState::QuestionAnswering;

        press(&mut app, KeyCode::Enter);
        assert_eq!(app.current_state, AppState::Loading);
        assert!(app.is_module_pending());
        assert!(app.pending_application.is_none());
    }

    #[tokio::test]
    async fn answering_moves_to_next_question() {
        let (mut app, _rx) = test_app();
        app.question_set = Some(QuestionSet::new(
            "t".to_string(),
            vec![question(QuestionType::Multiple), question(QuestionType::Binary)],
        ));
        app.current_state = AppState::QuestionAnswering;

        press(&mut app, KeyCode::Char('9')); // not an option
        assert_eq!(app.question_set.as_ref().unwrap().current_question_index, 0);
        press(&mut app, KeyCode::Char('c')); // letter for option 3
        let set = app.question_set.as_ref().unwrap();
        assert_eq!(set.questions[0].selected_answer.as_deref(), Some("3"));
        assert_eq!(set.current_question_index, 1);
    }

    #[tokio::test]
    async fn errors_show_popup_and_return_to_previous_screen() {
        let (mut app, _rx) = test_app();
        app.current_state = AppState::Learning;
        press(&mut app, KeyCode::Char('w'));
        assert_eq!(app.current_state, AppState::QuestionGeneration);

        let id = app.pending_questions.as_ref().unwrap().id;
        app.handle_task_result(TaskResult::Questions { id, result: Err(anyhow::anyhow!("boom")) });
        assert_eq!(app.current_state, AppState::Learning);
        assert!(app.last_error.as_deref().unwrap().contains("boom"));

        // Any key dismisses the popup without acting on it
        press(&mut app, KeyCode::Char('n'));
        assert!(app.last_error.is_none());
        assert_eq!(app.current_state, AppState::Learning);
    }

    #[tokio::test]
    async fn settings_return_to_previous_screen_and_are_blocked_while_busy() {
        let (mut app, _rx) = test_app();
        app.current_state = AppState::QuestionAnswering;
        press(&mut app, KeyCode::Char('s'));
        assert_eq!(app.current_state, AppState::Settings);
        press(&mut app, KeyCode::Esc);
        assert_eq!(app.current_state, AppState::QuestionAnswering);

        app.current_state = AppState::Loading;
        press(&mut app, KeyCode::Char('s'));
        assert_eq!(app.current_state, AppState::Loading);
    }

    #[tokio::test]
    async fn settings_keys_change_values() {
        let (mut app, _rx) = test_app();
        app.current_state = AppState::Settings;
        assert!(app.config().learning_resources.show_official_docs);
        press(&mut app, KeyCode::Enter);
        assert!(!app.config().learning_resources.show_official_docs);

        press(&mut app, KeyCode::Tab); // Content customization
        press(&mut app, KeyCode::Char('j')); // Explanation verbosity
        press(&mut app, KeyCode::Char('l'));
        assert_eq!(
            app.config().content_customization.explanation_verbosity,
            crate::config::ExplanationVerbosity::Detailed
        );
    }

    #[tokio::test]
    async fn model_picker_filters_and_selects() {
        let (mut app, _rx) = test_app();
        app.current_state = AppState::Settings;
        app.settings_section = SettingsSection::Model;
        press(&mut app, KeyCode::Enter);
        let id = app.pending_models.as_ref().unwrap().id;

        let models = vec![
            Model { id: "a/one:free".to_string(), name: "One".to_string(), is_free: true },
            Model { id: "b/two".to_string(), name: "Two".to_string(), is_free: false },
        ];
        app.handle_task_result(TaskResult::Models { id, result: Ok(models) });
        for c in "two".chars() {
            press(&mut app, KeyCode::Char(c)); // typing filters instead of quitting/navigating
        }
        assert!(!app.show_quit_confirmation);
        assert_eq!(app.model_picker.as_ref().unwrap().filtered().len(), 1);
        press(&mut app, KeyCode::Enter);
        assert!(app.model_picker.is_none());
        assert_eq!(app.config().model, "b/two");
    }

    fn type_text(app: &mut App, text: &str) {
        for c in text.chars() {
            press(app, KeyCode::Char(c));
        }
    }

    #[tokio::test]
    async fn source_selection_opens_topic_picker() {
        let (mut app, _rx) = test_app();
        app.current_state = AppState::IndexSelection;
        app.index_selection_cursor = 2; // The Rust Programming Language
        press(&mut app, KeyCode::Enter);

        let picker = app.topic_picker.as_ref().expect("topic picker opens");
        assert_eq!(picker.purpose, TopicPurpose::StartQuestions);
        assert_eq!(picker.choices()[0], TopicChoice::Random);
        assert!(picker.topics.iter().all(|t| t.min_level <= app.selected_level));
        assert!(app.pending_questions.is_none(), "nothing is generated before a topic is chosen");

        // Esc closes the picker and stays on the source selection
        press(&mut app, KeyCode::Esc);
        assert!(app.topic_picker.is_none());
        assert_eq!(app.current_state, AppState::IndexSelection);
    }

    #[tokio::test]
    async fn filtered_topic_is_used_for_questions() {
        let (mut app, _rx) = test_app();
        app.current_state = AppState::IndexSelection;
        app.index_selection_cursor = 2;
        press(&mut app, KeyCode::Enter);

        // Typing filters (and doesn't trigger global keys like 's' or 'q')
        type_text(&mut app, "struct");
        assert_eq!(app.current_state, AppState::IndexSelection);
        let choices = app.topic_picker.as_ref().unwrap().choices();
        let TopicChoice::Listed(expected) = choices[0].clone() else {
            panic!("expected a listed topic first, got {:?}", choices[0]);
        };
        assert!(expected.topic.to_lowercase().contains("struct"));
        assert!(matches!(choices.last(), Some(TopicChoice::Custom(text)) if text == "struct"));

        press(&mut app, KeyCode::Enter);
        assert!(app.topic_picker.is_none());
        assert_eq!(app.current_state, AppState::QuestionGeneration);
        assert_eq!(app.current_topic.as_ref(), Some(&expected));
    }

    #[tokio::test]
    async fn custom_topic_and_random_topic() {
        let (mut app, _rx) = test_app();
        app.current_state = AppState::IndexSelection;
        press(&mut app, KeyCode::Enter); // Rust Library (level 5 is enough)
        type_text(&mut app, "zzz lifetimes in async code");
        let choices = app.topic_picker.as_ref().unwrap().choices();
        assert_eq!(choices, [TopicChoice::Custom("zzz lifetimes in async code".to_string())]);
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.current_topic.as_ref().unwrap().topic, "zzz lifetimes in async code");
        assert_eq!(app.current_state, AppState::QuestionGeneration);

        // "Random" is the default entry
        app.current_state = AppState::IndexSelection;
        press(&mut app, KeyCode::Enter);
        press(&mut app, KeyCode::Enter);
        assert!(app.current_topic.as_ref().unwrap().topic.starts_with("Library: "));
    }

    #[tokio::test]
    async fn topic_picker_on_learning_screen_generates_module() {
        let (mut app, _rx) = test_app();
        app.current_state = AppState::Learning;
        app.selected_index = IndexType::RustByExample;
        press(&mut app, KeyCode::Char('t'));
        assert_eq!(app.topic_picker.as_ref().unwrap().purpose, TopicPurpose::NextModule);

        press(&mut app, KeyCode::Down); // first listed topic
        let TopicChoice::Listed(expected) = app.topic_picker.as_ref().unwrap().choices()[1].clone() else {
            panic!("expected a listed topic");
        };
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.current_state, AppState::Loading);
        assert!(app.is_module_pending());
        assert_eq!(app.current_topic.as_ref(), Some(&expected));
    }

    #[tokio::test]
    async fn history_browsing() {
        let (mut app, _rx) = test_app();
        app.current_state = AppState::Learning;
        for topic in ["one", "two", "three"] {
            app.add_to_history(module(topic));
        }
        press(&mut app, KeyCode::Char('['));
        press(&mut app, KeyCode::Char('['));
        assert_eq!(app.current_module.as_ref().unwrap().topic, "one");
        press(&mut app, KeyCode::Char('['));
        assert_eq!(app.current_module.as_ref().unwrap().topic, "one");
        press(&mut app, KeyCode::Char(']'));
        assert_eq!(app.current_module.as_ref().unwrap().topic, "two");
    }

    #[tokio::test]
    async fn scrolling_is_bounded() {
        let (mut app, _rx) = test_app();
        app.current_state = AppState::Learning;
        app.learning_scroll.max.set(5);
        app.learning_scroll.page.set(4);
        press(&mut app, KeyCode::Char('G'));
        assert_eq!(app.learning_scroll.offset, 5);
        press(&mut app, KeyCode::Char('j'));
        assert_eq!(app.learning_scroll.offset, 5);
        press(&mut app, KeyCode::PageUp);
        assert_eq!(app.learning_scroll.offset, 3);
        press(&mut app, KeyCode::Char('g'));
        assert_eq!(app.learning_scroll.offset, 0);
    }

    #[tokio::test]
    async fn failed_request_is_reported_through_the_channel() {
        let (mut app, mut rx) = test_app();
        app.current_state = AppState::Learning;
        press(&mut app, KeyCode::Char('n'));

        // No API key: the request fails and the error arrives as a task result
        let result = rx.recv().await.unwrap();
        app.handle_task_result(result);
        assert_eq!(app.current_state, AppState::Learning);
        assert!(app.last_error.as_deref().unwrap().contains("API key"));
    }
}
