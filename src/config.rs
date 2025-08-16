use serde::{Deserialize, Serialize};
use std::{env, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct JobSource {
    pub url: Option<String>,
    pub headers: Option<Vec<Header>>,
    pub wait: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub header_name: String,
    pub header_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub programming_languages: Vec<String>,
    pub job_sources: Vec<JobSource>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            programming_languages: vec!["rust".to_string()],
            job_sources: vec![JobSource {
                url: Some("https://jobicy.com/api/v2/remote-jobs".to_string()),
                headers: None,
                wait: None,
            }],
        }
    }
}

pub trait Testing {
    fn test_instance() -> Self;
}

impl Testing for Config {
    fn test_instance() -> Self {
        let test_work_time: u64 = env::var("OX_TEST_WORK")
            .unwrap_or_else(|_| "2000".to_string())
            .parse()
            .unwrap_or(2000);
        Self {
            programming_languages: vec!["rust".to_string()],
            job_sources: vec![JobSource {
                url: None,
                headers: None,
                wait: Some(test_work_time),
            }],
        }
    }
}

impl Config {
    pub fn config_dir() -> Option<PathBuf> {
        #[cfg(target_os = "windows")]
        return crate::windows::get_config_dir();

        #[cfg(target_os = "macos")]
        return crate::macos::get_config_dir();

        #[cfg(target_os = "linux")]
        return crate::linux::get_config_dir();

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        return dirs::config_dir().map(|d| d.join("oxidation"));
    }

    pub fn config_file_path() -> Option<PathBuf> {
        Self::config_dir().map(|dir| dir.join("config.toml"))
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = if let Some(config_path) = Self::config_file_path() {
            if config_path.exists() {
                let content = std::fs::read_to_string(config_path)?;
                toml::from_str(&content)?
            } else {
                Self::default()
            }
        } else {
            Self::default()
        };

        if config.programming_languages.is_empty() {
            config.programming_languages.push("rust".to_string());
        }

        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config_path) = Self::config_file_path() {
            if let Some(parent) = config_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let content = toml::to_string_pretty(self)?;
            std::fs::write(config_path, content)?;
        }
        Ok(())
    }

    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        let config = Self::default();
        config.save()?;
        if let Some(path) = Self::config_file_path() {
            println!("Config file created at: {}", path.display());
        }
        Ok(())
    }
}
