use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use toml_edit;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub model: String,
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
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::load()?;
        Ok(Self { config })
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn update_model(&mut self, model: String) -> Result<(), Box<dyn std::error::Error>> {
        self.config.model = model;
        self.config.save()
    }
}