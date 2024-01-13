use plugin_mod_log::config::ModLog;
use plugin_private_message::config::PrivateMessage;
use serde::Deserialize;
use std::fs;
use tracing::error;

#[derive(Deserialize)]
pub struct Config {
    pub lemmy: Lemmy,
    pub plugins: Plugins,
}

impl Config {
    pub fn load(filepath: &str) -> Option<Self> {
        let contents = match fs::read_to_string(filepath) {
            Ok(data) => data,
            Err(err) => {
                error!("failed to read file: {} -> {}", filepath, err);
                return None;
            }
        };
        let result = toml::from_str::<Self>(contents.as_str());
        match result {
            Ok(config) => Some(config),
            Err(err) => {
                error!("failed to parse config: {}", err);
                None
            }
        }
    }
}

#[derive(Deserialize)]
pub struct Lemmy {
    pub username: String,
    pub password: String,
    pub host: String,
}

#[derive(Deserialize)]
pub struct Plugins {
    #[serde(default)]
    pub mod_log: ModLog,
    #[serde(default)]
    pub private_message: PrivateMessage,
}
