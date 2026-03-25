use anyhow::Result;
use hdim_core::localization::{self, Localization};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Language {
    English,
    German,
}

impl Language {
    pub fn get_localization(&self) -> Localization {
        match self {
            Language::English => localization::en::get_localization(),
            Language::German => localization::de::get_localization(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub language: Language,
    pub theme: String, // Placeholder for future theme selection
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: Language::English,
            theme: "zinc".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        if let Some(config_path) = Self::get_config_path() {
            if let Ok(content) = fs::read_to_string(config_path) {
                if let Ok(config) = toml::from_str(&content) {
                    return config;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<()> {
        if let Some(config_path) = Self::get_config_path() {
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let content = toml::to_string_pretty(self)?;
            fs::write(config_path, content)?;
        }
        Ok(())
    }

    fn get_config_path() -> Option<PathBuf> {
        // Use directories or similar if cross-platform is needed,
        // for now simple home dir or relative.
        #[cfg(not(test))]
        {
            if let Some(mut path) = dirs::config_dir() {
                path.push("hdim");
                path.push("config.toml");
                return Some(path);
            }
        }
        None
    }
}
