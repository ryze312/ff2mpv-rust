use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use serde::Deserialize;

use crate::error::FF2MpvError;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub log_level: String,
    pub player_command: String,
    pub player_args: Vec<String>,
    pub ytdl_path: String,
    pub ytdl_cookies_from_browser: String,

    /// Optional flag that when set will force cookies to be re-exported via yt-dlp for the given
    /// browser ([`ytdl_cookies_from_browser`]) and pass them as input to the MPV command.
    /// You probably don't need ths and shouldn't as it creates a temporary file[^1] that contains
    /// cookies for the website you're trying to playback content on.
    ///
    /// [^1]: The temporary file is deleted as soon as this middle-man layer has completed executing
    /// the mpv command. For more details around the temporary file, see <https://docs.rs/tempfile/3.27.0/tempfile/struct.NamedTempFile.html#security-1>
    pub force_inject_cookies: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            player_command: "mpv".to_owned(),
            player_args: vec![String::from("--no-terminal"), String::from("--")],
            ytdl_path: "yt-dlp".to_string(),
            ytdl_cookies_from_browser: "firefox::none".to_string(),
            force_inject_cookies: None,
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
