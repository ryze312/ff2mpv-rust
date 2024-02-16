use std::env;
use std::io;
use std::process;
use serde_json::{self, json};

use crate::browser;
use crate::config::Config;
use crate::error::FF2MpvError;

pub enum Command {
    ShowHelp,
    ShowManifest,
    ShowManifestChromium,
    ValidateConfig,
    FF2Mpv,
}

impl Command {
    pub fn execute(&self) -> Result<(), FF2MpvError> {
        match self {
            Command::ShowHelp => Self::show_help(),
            Command::ShowManifest => Self::show_manifest(false),
            Command::ShowManifestChromium => Self::show_manifest(true),
            Command::ValidateConfig => Self::validate_config(),
            Command::FF2Mpv => Self::ff2mpv(),
        }
    }

    fn show_help() -> Result<(), FF2MpvError> {
        println!("Usage: ff2mpv-rust <command>");
        println!("Commands:");
        println!("  help: prints help message");
        println!("  manifest: prints manifest for Firefox configuration");
        println!("  manifest_chromium: prints manifest for Chromium/Chrome configuration");
        println!("  validate: checks configration file for validity");
        println!("Note: Invalid commands won't fail");
        println!("Note: It will assume that binary is called from browser, blocking for input");

        Ok(())
    }

    fn show_manifest(chromium: bool) -> Result<(), FF2MpvError> {
        let executable_path = env::current_exe()?;
        let allowed_keyvalue = if chromium {
            (
                "allowed_origins",
                "chrome-extension://ephjcajbkgplkjmelpglennepbpmdpjg",
            )
        } else {
            ("allowed_extensions", "ff2mpv@yossarian.net")
        };

        let manifest = json!({
            "name": "ff2mpv",
            "description": "ff2mpv's external manifest",
            "path": executable_path,
            "type": "stdio",
            allowed_keyvalue.0: [allowed_keyvalue.1]
        });

        let manifest = serde_json::to_string_pretty(&manifest)?;
        println!("{manifest}");

        Ok(())
    }

    fn validate_config() -> Result<(), FF2MpvError> {
        Config::parse_config_file()?;
        println!("Config is valid!");

        Ok(())
    }

    fn ff2mpv() -> Result<(), FF2MpvError> {
        let config = Config::build();
        let ff2mpv_message = browser::get_mpv_message()?;
        let args = [config.player_args, ff2mpv_message.options].concat();
        Command::launch_mpv(config.player_command, args, &ff2mpv_message.url)?;
        browser::send_reply()?;

        Ok(())
    }

    fn launch_mpv(command: String, args: Vec<String>, url: &str) -> Result<(), io::Error> {
        let mut command = process::Command::new(command);

        command.stdout(process::Stdio::null());
        command.stderr(process::Stdio::null());
        command.arg("--no-terminal");
        command.args(args);
        command.arg("--");
        command.arg(url);

        // NOTE: On Windows, browser spawns process into a Job object.
        // NOTE: We need to detach player from the job, so it won't get killed after we're done,
        // NOTE: See https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_messaging#closing_the_native_app
        #[cfg(target_family = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_BREAKAWAY_FROM_JOB: u32 = 0x01000000;

            command.creation_flags(CREATE_BREAKAWAY_FROM_JOB);
        }

        command.spawn()?;

        Ok(())
    }
}
