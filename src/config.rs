use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use toml_edit;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub model: String,
    pub learning_resources: LearningResources,
    pub content_customization: ContentCustomization,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LearningResources {
    pub show_official_docs: bool,
    pub show_community_resources: bool,
    pub show_crates_io: bool,
    pub show_github_repos: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentCustomization {
    pub code_complexity: CodeComplexity,
    pub explanation_verbosity: ExplanationVerbosity,
    pub focus_area: FocusArea,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CodeComplexity {
    Simple,
    Moderate,
    Complex,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ExplanationVerbosity {
    Concise,
    Moderate,
    Detailed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FocusArea {
    Concepts,
    CodeExamples,
    Exercises,
    Balanced,
}

impl Config {
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        let user_dirs = UserDirs::new().expect("Could not find user directories");
        let config_path = user_dirs.home_dir().join("rust-mentor.conf");

        let config: Config = if config_path.exists() {
            let mut config_file = fs::File::open(&config_path)?;
            let mut config_string = String::new();
            config_file.read_to_string(&mut config_string)?;
            toml_edit::de::from_str(&config_string)?
        } else {
            let default_config = Config {
                model: "google/gemma-3n-e4b-it:free".to_string(),
                learning_resources: LearningResources {
                    show_official_docs: true,
                    show_community_resources: true,
                    show_crates_io: true,
                    show_github_repos: true,
                },
                content_customization: ContentCustomization {
                    code_complexity: CodeComplexity::Moderate,
                    explanation_verbosity: ExplanationVerbosity::Moderate,
                    focus_area: FocusArea::Balanced,
                },
            };
            let toml = toml::to_string(&default_config)?;
            fs::write(&config_path, toml)?;
            default_config
        };

        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let user_dirs = UserDirs::new().expect("Could not find user directories");
        let config_path = user_dirs.home_dir().join("rust-mentor.conf");
        let toml = toml_edit::ser::to_string(self)?;
        fs::write(&config_path, toml)?;
        Ok(())
    }
}

pub struct ConfigService {
    config: Config,
}

impl ConfigService {
    pub fn new() -> Self {
        let config = Config::load();
        match config {
            Ok(config) => {
                tracing::info!("Loaded config: {:?}", config);
                ConfigService { config }
            },
            Err(err) => {
                tracing::error!("Failed to load config (~/rust-mentor.conf) - delete config file and rerun.");
                std::process::exit(-1);
            }
        }
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn update_model(&mut self, model: String) -> Result<(), Box<dyn std::error::Error>> {
        self.config.model = model;
        self.config.save()
    }

    // Learning resources methods
    pub fn get_learning_resources(&self) -> &LearningResources {
        &self.config.learning_resources
    }

    pub fn update_learning_resources(&mut self, resources: LearningResources) -> Result<(), Box<dyn std::error::Error>> {
        self.config.learning_resources = resources;
        self.config.save()
    }

    pub fn toggle_official_docs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.learning_resources.show_official_docs = !self.config.learning_resources.show_official_docs;
        self.config.save()
    }

    pub fn toggle_community_resources(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.learning_resources.show_community_resources = !self.config.learning_resources.show_community_resources;
        self.config.save()
    }

    pub fn toggle_crates_io(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.learning_resources.show_crates_io = !self.config.learning_resources.show_crates_io;
        self.config.save()
    }

    pub fn toggle_github_repos(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.learning_resources.show_github_repos = !self.config.learning_resources.show_github_repos;
        self.config.save()
    }

    // Content customization methods
    pub fn get_content_customization(&self) -> &ContentCustomization {
        &self.config.content_customization
    }

    pub fn update_content_customization(&mut self, customization: ContentCustomization) -> Result<(), Box<dyn std::error::Error>> {
        self.config.content_customization = customization;
        self.config.save()
    }

    pub fn update_code_complexity(&mut self, complexity: CodeComplexity) -> Result<(), Box<dyn std::error::Error>> {
        self.config.content_customization.code_complexity = complexity;
        self.config.save()
    }

    pub fn update_explanation_verbosity(&mut self, verbosity: ExplanationVerbosity) -> Result<(), Box<dyn std::error::Error>> {
        self.config.content_customization.explanation_verbosity = verbosity;
        self.config.save()
    }

    pub fn update_focus_area(&mut self, focus_area: FocusArea) -> Result<(), Box<dyn std::error::Error>> {
        self.config.content_customization.focus_area = focus_area;
        self.config.save()
    }

    pub fn cycle_code_complexity(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.content_customization.code_complexity = match self.config.content_customization.code_complexity {
            CodeComplexity::Simple => CodeComplexity::Moderate,
            CodeComplexity::Moderate => CodeComplexity::Complex,
            CodeComplexity::Complex => CodeComplexity::Simple,
        };
        self.config.save()
    }

    pub fn cycle_explanation_verbosity(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.content_customization.explanation_verbosity = match self.config.content_customization.explanation_verbosity {
            ExplanationVerbosity::Concise => ExplanationVerbosity::Moderate,
            ExplanationVerbosity::Moderate => ExplanationVerbosity::Detailed,
            ExplanationVerbosity::Detailed => ExplanationVerbosity::Concise,
        };
        self.config.save()
    }

    pub fn cycle_focus_area(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.content_customization.focus_area = match self.config.content_customization.focus_area {
            FocusArea::Concepts => FocusArea::CodeExamples,
            FocusArea::CodeExamples => FocusArea::Exercises,
            FocusArea::Exercises => FocusArea::Balanced,
            FocusArea::Balanced => FocusArea::Concepts,
        };
        self.config.save()
    }
}
