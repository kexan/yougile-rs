use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub api_token: String,
}

impl Config {
    pub fn load() -> io::Result<Self> {
        // Try to load from environment variables first
        if let (Ok(api_url), Ok(api_token)) = (std::env::var("YOUGILE_API_URL"), std::env::var("YOUGILE_API_TOKEN")) {
            return Ok(Config { api_url, api_token });
        }

        // Try to load from config file
        let config_path = Self::config_path()?;
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
            return Ok(config);
        }

        // Return default or error
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No configuration found. Set YOUGILE_API_URL and YOUGILE_API_TOKEN env vars or create config file.",
        ))
    }

    pub fn save(&self) -> io::Result<()> {
        let config_path = Self::config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        fs::write(config_path, content)?;
        Ok(())
    }

    fn config_path() -> io::Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Cannot find config directory"))?;
        Ok(config_dir.join("yougile-tui").join("config.toml"))
    }
}
