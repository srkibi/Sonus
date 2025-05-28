mod dialog;
pub use dialog::SettingsDialog;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// States for managing settings
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub auto_update: bool,
    pub buffer_size: u32,
    pub sample_rate: u32,
    #[serde(skip)]
    pub update_status: Option<String>,
    pub selected_input_device: Option<usize>,
    pub selected_output_device: Option<usize>,
}

impl Settings {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        if let Ok(contents) = fs::read_to_string(config_path) {
            if let Ok(settings) = serde_json::from_str(&contents) {
                return settings;
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        let config_path = Self::config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(config_path, contents)?;
        Ok(())
    }

    fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("sonus");
        path.push("settings.json");
        path
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            auto_update: false,
            buffer_size: 512,
            sample_rate: 44100,
            update_status: None,
            selected_input_device: None,
            selected_output_device: None,
        }
    }
}

// Available settings categories
#[derive(PartialEq)]
pub enum SettingsCategory {
    Updates,
    Audio,
}

