use std::env;
use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

use crate::error::FF2MpvError;

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    pub player_command: String,
    pub player_args: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            player_command: "mpv".to_owned(),
            player_args: vec![],
        }
    }
}

impl Config {
    const CONFIG_FILENAME: &'static str = "ff2mpv-rust.json";

    pub fn build() -> Self {
        Config::parse_config_file().unwrap_or_default()
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

    #[cfg(unix)]
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

        path.push(Self::CONFIG_FILENAME);
        path
    }

    #[cfg(windows)]
    fn get_config_location() -> PathBuf {
        let mut path = PathBuf::new();
        let appdata = env::var("APPDATA").unwrap();

        path.push(appdata);
        path.push(Self::CONFIG_FILENAME);
        path
    }
}
