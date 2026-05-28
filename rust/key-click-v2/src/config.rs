use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    pub keyboard_path: Option<String>,
    pub grid_size: u8,
    pub zoom_size: u8,
    pub overlay_opacity: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            keyboard_path: None,
            grid_size: 26,
            zoom_size: 3,
            overlay_opacity: 0.2,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = config_path();
        if !path.exists() {
            return Self::default();
        }
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
        toml::from_str(&text)
            .unwrap_or_else(|e| panic!("failed to parse {}: {e}", path.display()))
    }
}

fn config_path() -> PathBuf {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME not set");
            PathBuf::from(home).join(".config")
        });
    base.join("key-click").join("config.toml")
}
