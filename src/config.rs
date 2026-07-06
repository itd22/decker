use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use log::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub controlled_program: String,
    pub button_mappings: HashMap<String, ButtonMapping>,
    pub stick_mappings: HashMap<String, StickMapping>,
    pub deadzone: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonMapping {
    pub button: String,
    pub action: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickMapping {
    pub stick: String,
    pub action: String,
    pub key_positive: String,
    pub key_negative: String,
    pub threshold: f32,
}

impl Default for Config {
    fn default() -> Self {
        let mut button_mappings = HashMap::new();

        button_mappings.insert(
            "DPadUp".to_string(),
            ButtonMapping {
                button: "DPadUp".to_string(),
                action: "scroll_up".to_string(),
                keys: vec!["Up".to_string()],
            },
        );

        button_mappings.insert(
            "DPadDown".to_string(),
            ButtonMapping {
                button: "DPadDown".to_string(),
                action: "scroll_down".to_string(),
                keys: vec!["Down".to_string()],
            },
        );

        button_mappings.insert(
            "RB".to_string(),
            ButtonMapping {
                button: "RB".to_string(),
                action: "next_page".to_string(),
                keys: vec!["Right".to_string()],
            },
        );

        button_mappings.insert(
            "LB".to_string(),
            ButtonMapping {
                button: "LB".to_string(),
                action: "previous_page".to_string(),
                keys: vec!["Left".to_string()],
            },
        );

        button_mappings.insert(
            "A".to_string(),
            ButtonMapping {
                button: "A".to_string(),
                action: "zoom_in".to_string(),
                keys: vec!["ctrl".to_string(), "plus".to_string()],
            },
        );

        button_mappings.insert(
            "B".to_string(),
            ButtonMapping {
                button: "B".to_string(),
                action: "zoom_out".to_string(),
                keys: vec!["ctrl".to_string(), "minus".to_string()],
            },
        );

        button_mappings.insert(
            "Start".to_string(),
            ButtonMapping {
                button: "Start".to_string(),
                action: "toggle_fullscreen".to_string(),
                keys: vec!["F11".to_string()],
            },
        );

        Config {
            controlled_program: "okular".to_string(),
            button_mappings,
            stick_mappings: HashMap::new(),
            deadzone: 0.15,
        }
    }
}

pub fn config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Failed to determine config directory")?
        .join("decker");

    fs::create_dir_all(&config_dir)?;
    Ok(config_dir.join("config.toml"))
}

pub fn load_config() -> Result<Config> {
    let path = config_path()?;

    if path.exists() {
        info!("Loading config from {}", path.display());
        let content = fs::read_to_string(&path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    } else {
        info!("Config not found at {}, creating with defaults", path.display());
        let config = Config::default();
        save_config(&config)?;
        Ok(config)
    }
}

pub fn save_config(config: &Config) -> Result<()> {
    let path = config_path()?;
    let content = toml::to_string_pretty(config)?;
    fs::write(&path, content)?;
    info!("Config saved to {}", path.display());
    Ok(())
}
