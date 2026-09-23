// src/config.rs
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use strum::{Display, EnumIter, IntoEnumIterator};

pub const DEFAULT_MODEL: &str = "google/gemma-3n-e4b-it:free";
pub const MIN_QUESTIONS: usize = 3;
pub const MAX_QUESTIONS: usize = 10;

/// Steps an enum value to the next (or previous) variant, wrapping around
pub trait Cycle: IntoEnumIterator + PartialEq + Copy {
    fn cycle(self, forward: bool) -> Self {
        let all: Vec<Self> = Self::iter().collect();
        let index = all.iter().position(|v| *v == self).unwrap_or(0);
        let len = all.len();
        let next = if forward { (index + 1) % len } else { (index + len - 1) % len };
        all[next]
    }
}

impl<T: IntoEnumIterator + PartialEq + Copy> Cycle for T {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct Config {
    pub model: String,
    /// Where Cargo projects are created; empty means the current directory
    pub projects_dir: String,
    pub learning_resources: LearningResources,
    pub content_customization: ContentCustomization,
    pub question_generator_settings: QuestionGeneratorSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct LearningResources {
    pub show_official_docs: bool,
    pub show_community_resources: bool,
    pub show_crates_io: bool,
    pub show_github_repos: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(default)]
pub struct ContentCustomization {
    pub code_complexity: CodeComplexity,
    pub explanation_verbosity: ExplanationVerbosity,
    pub focus_area: FocusArea,
    pub learning_goal: LearningGoal,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Default, Display, EnumIter)]
pub enum CodeComplexity {
    Simple,
    #[default]
    Moderate,
    Complex,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Default, Display, EnumIter)]
pub enum ExplanationVerbosity {
    Concise,
    #[default]
    Moderate,
    Detailed,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Default, Display, EnumIter)]
pub enum FocusArea {
    Concepts,
    #[strum(to_string = "Code Examples")]
    CodeExamples,
    Exercises,
    #[default]
    Balanced,
}

/// Which kinds of questions the question generator asks
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Default, Display, EnumIter)]
pub enum QuestionStyle {
    #[default]
    Mixed,
    #[strum(to_string = "Yes/No")]
    YesNo,
    #[strum(to_string = "Multiple Choice")]
    MultipleChoice,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct QuestionGeneratorSettings {
    pub num_questions: usize,
    pub question_style: QuestionStyle,
    pub enable_application_generation: bool,
}

// Learning goals for personalized learning paths.
// Variant names are stored in the config file, so they must not be renamed.
#[allow(clippy::upper_case_acronyms)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Default, Display, EnumIter)]
pub enum LearningGoal {
    #[strum(to_string = "AR/VR")]
    ARVR,
    #[strum(to_string = "Async Programming")]
    AsyncProgramming,
    #[strum(to_string = "Big Data")]
    BigData,
    Bioinformatics,
    Bitcoin,
    #[strum(to_string = "Cloud Computing")]
    CloudComputing,
    #[strum(to_string = "Computer Vision")]
    ComputerVision,
    Concurrency,
    Cuda, // Compute Unified Device Architecture (for GPU programming)
    Cybersecurity,
    DICOM, // Medical Imaging standard
    #[strum(to_string = "Data Science")]
    DataScience,
    Databases,
    #[strum(to_string = "Deep Learning")]
    DeepLearning,
    DevOps,
    #[strum(to_string = "Distributed Systems")]
    DistributedSystems,
    #[strum(to_string = "Edge Computing")]
    EdgeComputing,
    #[strum(to_string = "Embedded Systems")]
    EmbeddedSystems,
    #[strum(to_string = "Ethical AI")]
    EthicalAI,
    GANs, // Generative Adversarial Networks
    GPU,
    #[default]
    General,
    Graphics,
    HL7, // Medical data exchange standard
    #[strum(to_string = "Image Processing")]
    ImageProcessing,
    #[strum(to_string = "Machine Learning")]
    MachineLearning,
    #[strum(to_string = "Medical Imaging")]
    MedicalImaging,
    #[strum(to_string = "Microservices")]
    MicroServices,
    MicroVM,
    #[strum(to_string = "Natural Language Processing")]
    NaturalLanguageProcessing,
    Networking,
    #[strum(to_string = "Operating Systems")]
    OperatingSystems,
    PyTorch,
    #[strum(to_string = "Quantum Computing")]
    QuantumComputing,
    ROS, // Robot Operating System
    #[strum(to_string = "Reinforcement Learning")]
    ReinforcementLearning,
    Robotics,
    SLAM, // Simultaneous Localization and Mapping
    #[strum(to_string = "Sensor Fusion")]
    SensorFusion,
    #[strum(to_string = "Systems Programming")]
    SystemsProgramming,
    TUI, // Terminal User Interface
    Transformers,
    #[strum(to_string = "User Interface")]
    UserInterface,
    #[strum(to_string = "Web Development")]
    WebDevelopment,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            model: DEFAULT_MODEL.to_string(),
            projects_dir: String::new(),
            learning_resources: LearningResources::default(),
            content_customization: ContentCustomization::default(),
            question_generator_settings: QuestionGeneratorSettings::default(),
        }
    }
}

impl Default for LearningResources {
    fn default() -> Self {
        LearningResources {
            show_official_docs: true,
            show_community_resources: true,
            show_crates_io: true,
            show_github_repos: true,
        }
    }
}

impl Default for QuestionGeneratorSettings {
    fn default() -> Self {
        QuestionGeneratorSettings {
            num_questions: 5,
            question_style: QuestionStyle::default(),
            enable_application_generation: true,
        }
    }
}

impl Config {
    /// Parses a config file; missing fields take their default values
    pub fn from_toml(text: &str) -> Result<Config> {
        let mut config: Config = toml::from_str(text)?;
        config.question_generator_settings.num_questions =
            config.question_generator_settings.num_questions.clamp(MIN_QUESTIONS, MAX_QUESTIONS);
        if config.model.trim().is_empty() {
            config.model = DEFAULT_MODEL.to_string();
        }
        Ok(config)
    }

    pub fn to_toml(&self) -> Result<String> {
        Ok(toml::to_string(self)?)
    }

    /// Directory for new Cargo projects (`~` is expanded)
    pub fn projects_dir(&self) -> PathBuf {
        let dir = self.projects_dir.trim();
        if dir.is_empty() {
            return std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        }
        match (dir.strip_prefix("~/"), directories::UserDirs::new()) {
            (Some(rest), Some(user_dirs)) => user_dirs.home_dir().join(rest),
            _ => PathBuf::from(dir),
        }
    }
}

/// Default location of the config file, e.g. `~/.config/rust-mentor/config.toml` on Linux
/// or `~/Library/Application Support/rust-mentor/config.toml` on macOS
pub fn default_config_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("", "", "rust-mentor").map(|dirs| dirs.config_dir().join("config.toml"))
}

/// Config file location used by earlier versions
fn legacy_config_path() -> Option<PathBuf> {
    directories::UserDirs::new().map(|dirs| dirs.home_dir().join("rust-mentor.conf"))
}

/// Holds the configuration and persists every change to the config file
pub struct ConfigService {
    config: Config,
    path: Option<PathBuf>, // None: in-memory only
}

impl ConfigService {
    /// Loads the config file, creating it with defaults (or migrating the legacy
    /// `~/rust-mentor.conf`) if it doesn't exist yet
    pub fn load() -> Result<Self> {
        let path = default_config_path().context("Could not determine the config directory")?;
        Self::load_from(&path, legacy_config_path().as_deref())
    }

    fn load_from(path: &Path, legacy_path: Option<&Path>) -> Result<Self> {
        let config = if path.exists() {
            let text =
                fs::read_to_string(path).with_context(|| format!("Failed to read config file {}", path.display()))?;
            Config::from_toml(&text)
                .with_context(|| format!("Invalid config file {} - fix or delete it", path.display()))?
        } else if let Some(legacy_path) = legacy_path.filter(|p| p.exists()) {
            tracing::info!("Migrating config from {}", legacy_path.display());
            let text = fs::read_to_string(legacy_path)?;
            Config::from_toml(&text).unwrap_or_else(|err| {
                tracing::warn!("Ignoring invalid legacy config {}: {}", legacy_path.display(), err);
                Config::default()
            })
        } else {
            Config::default()
        };

        let service = ConfigService { config, path: Some(path.to_path_buf()) };
        // Write back so new fields appear in the file (and the file exists)
        service.save()?;
        tracing::info!("Loaded config from {}: {:?}", path.display(), service.config);
        Ok(service)
    }

    /// A config service that is not backed by a file
    #[cfg(test)]
    pub fn in_memory(config: Config) -> Self {
        ConfigService { config, path: None }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Applies a change and saves the config file
    pub fn update(&mut self, change: impl FnOnce(&mut Config)) -> Result<()> {
        change(&mut self.config);
        self.save()
    }

    fn save(&self) -> Result<()> {
        let Some(path) = &self.path else {
            return Ok(());
        };
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).with_context(|| format!("Failed to create config directory {}", dir.display()))?;
        }
        fs::write(path, self.config.to_toml()?)
            .with_context(|| format!("Failed to write config file {}", path.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_fields_use_defaults() {
        let config = Config::from_toml("model = \"some/model\"\n").unwrap();
        assert_eq!(config.model, "some/model");
        assert_eq!(config.learning_resources, LearningResources::default());
        assert_eq!(config.question_generator_settings.num_questions, 5);
    }

    #[test]
    fn legacy_config_is_readable() {
        // Format written by earlier versions (includes a field that no longer exists)
        let legacy = r#"
model = "google/gemini-flash"

[learning_resources]
show_official_docs = false
show_community_resources = true
show_crates_io = true
show_github_repos = true

[content_customization]
code_complexity = "Complex"
explanation_verbosity = "Concise"
focus_area = "CodeExamples"
learning_goal = "ARVR"

[question_generator_settings]
num_questions = 42
default_question_type = "Multiple"
enable_application_generation = true
"#;
        let config = Config::from_toml(legacy).unwrap();
        assert_eq!(config.model, "google/gemini-flash");
        assert!(!config.learning_resources.show_official_docs);
        assert_eq!(config.content_customization.code_complexity, CodeComplexity::Complex);
        assert_eq!(config.content_customization.learning_goal, LearningGoal::ARVR);
        assert_eq!(config.question_generator_settings.num_questions, MAX_QUESTIONS);
    }

    #[test]
    fn round_trips_through_toml() {
        let mut config = Config::default();
        config.content_customization.focus_area = FocusArea::Exercises;
        config.question_generator_settings.question_style = QuestionStyle::YesNo;
        let parsed = Config::from_toml(&config.to_toml().unwrap()).unwrap();
        assert_eq!(parsed, config);
    }

    #[test]
    fn cycle_wraps_in_both_directions() {
        assert_eq!(CodeComplexity::Complex.cycle(true), CodeComplexity::Simple);
        assert_eq!(CodeComplexity::Simple.cycle(false), CodeComplexity::Complex);
        assert_eq!(LearningGoal::WebDevelopment.cycle(true), LearningGoal::ARVR);
        assert_eq!(LearningGoal::General.cycle(true), LearningGoal::Graphics);
    }

    #[test]
    fn display_names() {
        assert_eq!(LearningGoal::ARVR.to_string(), "AR/VR");
        assert_eq!(LearningGoal::WebDevelopment.to_string(), "Web Development");
        assert_eq!(FocusArea::CodeExamples.to_string(), "Code Examples");
    }

    #[test]
    fn creates_and_migrates_config_files() {
        let dir = std::env::temp_dir().join(format!("rust-mentor-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let legacy = dir.join("rust-mentor.conf");
        fs::write(&legacy, "model = \"legacy/model\"\n").unwrap();
        let path = dir.join("config").join("config.toml");

        let service = ConfigService::load_from(&path, Some(&legacy)).unwrap();
        assert_eq!(service.config().model, "legacy/model");
        assert!(path.exists());

        let mut service = ConfigService::load_from(&path, None).unwrap();
        service.update(|c| c.model = "new/model".to_string()).unwrap();
        let reloaded = ConfigService::load_from(&path, None).unwrap();
        assert_eq!(reloaded.config().model, "new/model");

        fs::remove_dir_all(&dir).unwrap();
    }
}
