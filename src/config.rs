use std::env;
use std::fs;
use std::io::ErrorKind;
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
            player_args: vec![String::from("--no-terminal"), String::from("--")],
        }
    }
}

impl Config {
    const CONFIG_FILENAME: &str = "ff2mpv-rust.json";

    #[must_use]
    pub fn build() -> Self {
        match Config::parse_config_file() {
            Ok(config) => config,

            Err(FF2MpvError::NoConfig) => {
                eprintln!("Config not found, using defaults");
                Config::default()
            }

            Err(e) => {
                eprintln!("Error occured while parsing config, using defaults");
                eprintln!("{e}");

                Config::default()
            }
        }
    }

    pub fn parse_config_file() -> Result<Self, FF2MpvError> {
        let config_path = Config::get_config_location();
        let string = match fs::read_to_string(config_path) {
            Ok(string) => string,

            Err(e) if e.kind() == ErrorKind::NotFound => {
                return Err(FF2MpvError::NoConfig);
            }

            Err(e) => {
                return Err(FF2MpvError::IOError(e));
            }
        };

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
