use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bgcolor: Option<String>,
    pub fgcolor: Option<String>,
    pub aliases: Option<HashMap<String, String>>,
    pub env: Option<HashMap<String, String>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bgcolor: Some("gray".to_string()),
            fgcolor: Some("white".to_string()),
            aliases: Some(HashMap::new()),
            env: Some(HashMap::new()),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".controlconfig");

        if !config_path.exists() {
            // Создаём конфиг по умолчанию
            let default = Config::default();
            let _ = default.save(); // пытаемся сохранить
            return default;
        }

        match fs::read_to_string(&config_path) {
            Ok(content) => {
                toml::from_str(&content).unwrap_or_else(|e| {
                    eprintln!("Error parsing config: {}", e);
                    Config::default()
                })
            }
            Err(e) => {
                eprintln!("Error reading config: {}", e);
                Config::default()
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let config_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".controlconfig");

        let toml_string = toml::to_string(self).map_err(|e| e.to_string())?;
        fs::write(config_path, toml_string).map_err(|e| e.to_string())?;
        Ok(())
    }
}