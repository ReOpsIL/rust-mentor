use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use toml_edit;
use crate::app::LearningGoal;
use crate::question_generator::QuestionType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub model: String,
    pub learning_resources: LearningResources,
    pub content_customization: ContentCustomization,
    pub question_generator_settings: QuestionGeneratorSettings,
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
    pub learning_goal: LearningGoal,
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

const MIN_QUESTIONS: usize = 3;
const MAX_QUESTIONS: usize =  10;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuestionGeneratorSettings {
    pub num_questions: usize,
    pub default_question_type: QuestionType,
    pub enable_application_generation: bool,
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
                    learning_goal: LearningGoal::General,
                },
                question_generator_settings: QuestionGeneratorSettings {
                    num_questions: 5,
                    default_question_type: QuestionType::Multiple,
                    enable_application_generation: true,
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
            Err(_) => {
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

    pub fn increment_num_questions(&mut self) {
        if self.config.question_generator_settings.num_questions < MAX_QUESTIONS {
            self.config.question_generator_settings.num_questions += 1;
        }
    }
    pub fn decrement_num_questions(&mut self) {
        if self.config.question_generator_settings.num_questions > MIN_QUESTIONS {
            self.config.question_generator_settings.num_questions -= 1;
        }
    }

    pub fn cycle_code_complexity(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.content_customization.code_complexity = match self.config.content_customization.code_complexity {
            CodeComplexity::Simple => CodeComplexity::Moderate,
            CodeComplexity::Moderate => CodeComplexity::Complex,
            CodeComplexity::Complex => CodeComplexity::Simple,
        };
        self.config.save()
    }

    pub fn cycle_code_complexity_reverse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.content_customization.code_complexity = match self.config.content_customization.code_complexity {
            CodeComplexity::Simple => CodeComplexity::Complex,
            CodeComplexity::Moderate => CodeComplexity::Simple,
            CodeComplexity::Complex => CodeComplexity::Moderate,
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

    pub fn cycle_explanation_verbosity_reverse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.content_customization.explanation_verbosity = match self.config.content_customization.explanation_verbosity {
            ExplanationVerbosity::Concise => ExplanationVerbosity::Detailed,
            ExplanationVerbosity::Moderate => ExplanationVerbosity::Concise,
            ExplanationVerbosity::Detailed => ExplanationVerbosity::Moderate,
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

    pub fn cycle_focus_area_reverse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.content_customization.focus_area = match self.config.content_customization.focus_area {
            FocusArea::Concepts => FocusArea::Balanced,
            FocusArea::CodeExamples => FocusArea::Concepts,
            FocusArea::Exercises => FocusArea::CodeExamples,
            FocusArea::Balanced => FocusArea::Exercises,
        };
        self.config.save()
    }

    pub fn cycle_learning_goal(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.content_customization.learning_goal = match self.config.content_customization.learning_goal {
            LearningGoal::ARVR => LearningGoal::AsyncProgramming,
            LearningGoal::AsyncProgramming => LearningGoal::BigData,
            LearningGoal::BigData => LearningGoal::Bioinformatics,
            LearningGoal::Bioinformatics => LearningGoal::Bitcoin,
            LearningGoal::Bitcoin => LearningGoal::CloudComputing,
            LearningGoal::CloudComputing => LearningGoal::ComputerVision,
            LearningGoal::ComputerVision => LearningGoal::Concurrency,
            LearningGoal::Concurrency => LearningGoal::Cuda,
            LearningGoal::Cuda => LearningGoal::Cybersecurity,
            LearningGoal::Cybersecurity => LearningGoal::DICOM,
            LearningGoal::DICOM => LearningGoal::DataScience,
            LearningGoal::DataScience => LearningGoal::Databases,
            LearningGoal::Databases => LearningGoal::DeepLearning,
            LearningGoal::DeepLearning => LearningGoal::DevOps,
            LearningGoal::DevOps => LearningGoal::DistributedSystems,
            LearningGoal::DistributedSystems => LearningGoal::EdgeComputing,
            LearningGoal::EdgeComputing => LearningGoal::EmbeddedSystems,
            LearningGoal::EmbeddedSystems => LearningGoal::EthicalAI,
            LearningGoal::EthicalAI => LearningGoal::GANs,
            LearningGoal::GANs => LearningGoal::GPU,
            LearningGoal::GPU => LearningGoal::General,
            LearningGoal::General => LearningGoal::Graphics,
            LearningGoal::Graphics => LearningGoal::HL7,
            LearningGoal::HL7 => LearningGoal::ImageProcessing,
            LearningGoal::ImageProcessing => LearningGoal::MachineLearning,
            LearningGoal::MachineLearning => LearningGoal::MedicalImaging,
            LearningGoal::MedicalImaging => LearningGoal::MicroServices,
            LearningGoal::MicroServices => LearningGoal::MicroVM,
            LearningGoal::MicroVM => LearningGoal::NaturalLanguageProcessing,
            LearningGoal::NaturalLanguageProcessing => LearningGoal::Networking,
            LearningGoal::Networking => LearningGoal::OperatingSystems,
            LearningGoal::OperatingSystems => LearningGoal::PyTorch,
            LearningGoal::PyTorch => LearningGoal::QuantumComputing,
            LearningGoal::QuantumComputing => LearningGoal::ROS,
            LearningGoal::ROS => LearningGoal::ReinforcementLearning,
            LearningGoal::ReinforcementLearning => LearningGoal::Robotics,
            LearningGoal::Robotics => LearningGoal::SLAM,
            LearningGoal::SLAM => LearningGoal::SensorFusion,
            LearningGoal::SensorFusion => LearningGoal::SystemsProgramming,
            LearningGoal::SystemsProgramming => LearningGoal::TUI,
            LearningGoal::TUI => LearningGoal::Transformers,
            LearningGoal::Transformers => LearningGoal::UserInterface,
            LearningGoal::UserInterface => LearningGoal::WebDevelopment,
            LearningGoal::WebDevelopment => LearningGoal::ARVR,
        };
        self.config.save()
    }

    pub fn cycle_learning_goal_reverse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.content_customization.learning_goal = match self.config.content_customization.learning_goal {
            LearningGoal::ARVR => LearningGoal::WebDevelopment,
            LearningGoal::AsyncProgramming => LearningGoal::ARVR,
            LearningGoal::BigData => LearningGoal::AsyncProgramming,
            LearningGoal::Bioinformatics => LearningGoal::BigData,
            LearningGoal::Bitcoin => LearningGoal::Bioinformatics,
            LearningGoal::CloudComputing => LearningGoal::Bitcoin,
            LearningGoal::ComputerVision => LearningGoal::CloudComputing,
            LearningGoal::Concurrency => LearningGoal::ComputerVision,
            LearningGoal::Cuda => LearningGoal::Concurrency,
            LearningGoal::Cybersecurity => LearningGoal::Cuda,
            LearningGoal::DICOM => LearningGoal::Cybersecurity,
            LearningGoal::DataScience => LearningGoal::DICOM,
            LearningGoal::Databases => LearningGoal::DataScience,
            LearningGoal::DeepLearning => LearningGoal::Databases,
            LearningGoal::DevOps => LearningGoal::DeepLearning,
            LearningGoal::DistributedSystems => LearningGoal::DevOps,
            LearningGoal::EdgeComputing => LearningGoal::DistributedSystems,
            LearningGoal::EmbeddedSystems => LearningGoal::EdgeComputing,
            LearningGoal::EthicalAI => LearningGoal::EmbeddedSystems,
            LearningGoal::GANs => LearningGoal::EthicalAI,
            LearningGoal::GPU => LearningGoal::GANs,
            LearningGoal::General => LearningGoal::GPU,
            LearningGoal::Graphics => LearningGoal::General,
            LearningGoal::HL7 => LearningGoal::Graphics,
            LearningGoal::ImageProcessing => LearningGoal::HL7,
            LearningGoal::MachineLearning => LearningGoal::ImageProcessing,
            LearningGoal::MedicalImaging => LearningGoal::MachineLearning,
            LearningGoal::MicroServices => LearningGoal::MedicalImaging,
            LearningGoal::MicroVM => LearningGoal::MicroServices,
            LearningGoal::NaturalLanguageProcessing => LearningGoal::MicroVM,
            LearningGoal::Networking => LearningGoal::NaturalLanguageProcessing,
            LearningGoal::OperatingSystems => LearningGoal::Networking,
            LearningGoal::PyTorch => LearningGoal::OperatingSystems,
            LearningGoal::QuantumComputing => LearningGoal::PyTorch,
            LearningGoal::ROS => LearningGoal::QuantumComputing,
            LearningGoal::ReinforcementLearning => LearningGoal::ROS,
            LearningGoal::Robotics => LearningGoal::ReinforcementLearning,
            LearningGoal::SLAM => LearningGoal::Robotics,
            LearningGoal::SensorFusion => LearningGoal::SLAM,
            LearningGoal::SystemsProgramming => LearningGoal::SensorFusion,
            LearningGoal::TUI => LearningGoal::SystemsProgramming,
            LearningGoal::Transformers => LearningGoal::TUI,
            LearningGoal::UserInterface => LearningGoal::Transformers,
            LearningGoal::WebDevelopment => LearningGoal::UserInterface,
        };
        self.config.save()
    }

    // Question generator settings methods
    pub fn get_question_generator_settings(&self) -> &QuestionGeneratorSettings {
        &self.config.question_generator_settings
    }

    pub fn update_question_generator_settings(&mut self, settings: QuestionGeneratorSettings) -> Result<(), Box<dyn std::error::Error>> {
        self.config.question_generator_settings = settings;
        self.config.save()
    }

    pub fn update_num_questions(&mut self, num_questions: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.config.question_generator_settings.num_questions = num_questions;
        self.config.save()
    }

    pub fn update_default_question_type(&mut self, question_type: QuestionType) -> Result<(), Box<dyn std::error::Error>> {
        self.config.question_generator_settings.default_question_type = question_type;
        self.config.save()
    }

    pub fn toggle_application_generation(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.question_generator_settings.enable_application_generation = !self.config.question_generator_settings.enable_application_generation;
        self.config.save()
    }

    pub fn cycle_question_type(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.question_generator_settings.default_question_type = match self.config.question_generator_settings.default_question_type {
            QuestionType::Binary => QuestionType::Multiple,
            QuestionType::Multiple => QuestionType::Binary,
        };
        self.config.save()
    }
}
