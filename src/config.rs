use std::env;
use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

use crate::error::FF2MpvError;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_player_command")]
    pub player_command: String,

    #[serde(default = "default_player_args")]
    pub player_args: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            player_command: default_player_command(),
            player_args: default_player_args(),
        }
    }
}

impl Config {
    pub fn build() -> Self {
        if let Ok(config) = Config::parse_config_file() {
            config
        } else {
            Config::default()
        }
    }

    pub fn parse_config_file() -> Result<Self, FF2MpvError> {
        let config_path = Config::get_config_location();
        if !config_path.exists() {
            return Err(FF2MpvError::NoConfig);
        }

        let string = fs::read_to_string(config_path)?;
        let config = serde_json::from_str(&string)?;

        Ok(config)
    }

    #[cfg(target_family = "unix")]
    fn get_config_location() -> PathBuf {
        let mut path = PathBuf::new();

        if let Ok(home) = env::var("XDG_CONFIG_HOME") {
            path.push(home);
        } else if let Ok(home) = env::var("HOME") {
            path.push(home);
            path.push(".config");
        } else {
            path.push("/etc");
        }

        path.push("ff2mpv-rust.json");
        path
    }

    #[cfg(target_family = "windows")]
    fn get_config_location() -> PathBuf {
        let mut path = PathBuf::new();
        let appdata = env::var("APPDATA").unwrap();

        path.push(appdata);
        path.push("ff2mpv-rust.json");
        path
    }
}

fn default_player_command() -> String {
    "mpv".to_owned()
}

fn default_player_args() -> Vec<String> {
    vec![]
}
