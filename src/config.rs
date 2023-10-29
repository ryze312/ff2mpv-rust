use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

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
    pub const FILENAME: &str = "ff2mpv-rust.json";

    pub fn build() -> Self {
        Config::parse_config_file().unwrap_or_default()
    }

    pub fn parse_config_file() -> Result<Self, FF2MpvError> {
        let config_path = Config::get_config_location()?;
        let string = fs::read_to_string(config_path)?;
        let config = serde_json::from_str(&string)?;

        Ok(config)
    }

    /// Returns a *valid* config path, i.e. it exists and is readable by the
    /// current user. If the config path is not valid, an [`FF2MpvError`] is
    /// returned.
    fn get_config_location() -> Result<PathBuf, FF2MpvError> {
        let config_dir = if cfg!(target_family = "unix") {
            if let Ok(home) = env::var("XDG_CONFIG_HOME") {
                PathBuf::from(home)
            } else if let Ok(home) = env::var("HOME") {
                PathBuf::from(home).join(".config")
            } else {
                PathBuf::from("/etc")
            }
        } else if cfg!(target_family = "windows") {
            env::var("APPDATA")
                .map_err(|_| FF2MpvError::NoConfig)?
                .into()
        } else {
            unimplemented!("This platform is not supported")
        };

        let path = config_dir.join(Config::FILENAME);
        match path.try_exists() {
            Ok(true) => Ok(path),
            // Broken symbolic link to config file.
            Ok(false) => Err(FF2MpvError::NoConfig),
            Err(err) => Err(FF2MpvError::IOError(err)),
        }
    }
}
