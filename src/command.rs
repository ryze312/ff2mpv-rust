use std::{env, io, path::Path, process};

use serde_json::json;
use tempfile::NamedTempFile;
use tracing::{debug, error, trace};

use crate::{browser, config::Config, error::FF2MpvError};

pub enum Command {
    ShowHelp,
    ShowManifest,
    ShowManifestChromium,
    ValidateConfig,
    FF2Mpv,
}

#[allow(clippy::unnecessary_wraps, reason = "More readable in show_help")]
impl Command {
    pub fn execute(&self, config: &Config) -> Result<(), FF2MpvError> {
        match self {
            Command::ShowHelp => Self::show_help(),
            Command::ShowManifest => Self::show_manifest(false),
            Command::ShowManifestChromium => Self::show_manifest(true),
            Command::ValidateConfig => Self::validate_config(),
            Command::FF2Mpv => Self::ff2mpv(config),
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
                "chrome-extension://ephjcajbkgplkjmelpglennepbpmdpjg/",
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

    fn ff2mpv(config: &Config) -> Result<(), FF2MpvError> {
        let ff2mpv_message = browser::get_mpv_message()?;
        debug!("Parsed message: {ff2mpv_message:?}");

        let mut extra_args: Vec<String> = vec![
            format!(
                "--ytdl-raw-options-append=cookies-from-browser={}",
                config.ytdl_cookies_from_browser
            ),
            format!("--script-opts=ytdl_hook-ytdl_path={}", config.ytdl_path),
        ];

        if config.force_inject_cookies.is_some_and(|v| v)
            && let Some(cookie_file) = Command::export_cookies(
                &config.ytdl_path,
                &config.ytdl_cookies_from_browser,
                &ff2mpv_message.url,
            )
            && let Some(header) =
                Command::build_cookie_argument(cookie_file.path(), &ff2mpv_message.url)
        {
            extra_args.push(format!("--http-header-fields-append=Cookie: {header}"));
        }

        let args = [
            extra_args,
            config.player_args.clone(),
            ff2mpv_message.options,
        ]
        .concat();
        if let Err(e) =
            Command::launch_mpv(config.player_command.clone(), args, &ff2mpv_message.url)
        {
            debug!("Failed to launch mpv: {e}");
            return Err(e.into());
        }

        browser::send_reply()?;
        trace!("Reply sent to browser");

        Ok(())
    }

    fn export_cookies(ytdl: &str, browser: &str, url: &str) -> Option<NamedTempFile> {
        use std::io::Write;

        let mut cookies = match NamedTempFile::new() {
            Ok(f) => f,
            Err(err) => {
                debug!("Failed to create temporary file: {err:?}");
                return None;
            }
        };

        // yt-dlp expects a header to exist before writing
        let _ = writeln!(cookies, "# Netscape HTTP Cookie File");
        let cookie_filepath = cookies.path().to_string_lossy();

        let output = process::Command::new(ytdl)
            .args([
                "--cookies-from-browser",
                browser,
                "--cookies",
                cookie_filepath.as_ref(),
                "--skip-download",
                "--no-warnings",
                url,
            ])
            .stdout(process::Stdio::null())
            .output()
            .expect("command to be valid");

        if !output.stderr.is_empty() {
            error!(
                "yt-dlp command returned an error: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return None;
        }

        Some(cookies)
    }

    fn build_cookie_argument(cookie_file: &Path, url: &str) -> Option<String> {
        let url_parts = url.split('/').collect::<Vec<&str>>();
        let host = url_parts[2].split(':').next()?;

        let content = std::fs::read_to_string(cookie_file).ok()?;
        let cookies: Vec<String> = content
            .lines()
            .filter(|l| !l.starts_with('#') && !l.is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.splitn(7, '\t').collect();
                if parts.len() < 7 {
                    return None;
                }
                let domain = parts[0].trim_start_matches('.');
                if domain == host {
                    Some(format!("{}={}", parts[5], parts[6]))
                } else {
                    None
                }
            })
            .collect();

        if cookies.is_empty() {
            debug!("no cookies found for host {host}");
            None
        } else {
            debug!("Configured {} cookies for {host}", cookies.len());
            Some(cookies.join("; "))
        }
    }

    fn launch_mpv(command: String, args: Vec<String>, url: &str) -> Result<(), io::Error> {
        let mut command = process::Command::new(command);

        command.stdout(process::Stdio::null());
        command.stderr(process::Stdio::null());
        command.args(args);
        command.arg(url);

        Command::detach_mpv(&mut command);

        // WARN: Do not log `command` as it contains secrets
        debug!("Launching mpv");
        command.spawn()?;

        Ok(())
    }

    // NOTE: Make sure the subprocess is not killed.
    //       See https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_messaging#closing_the_native_app

    #[cfg(unix)]
    fn detach_mpv(command: &mut process::Command) {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }

    #[cfg(windows)]
    fn detach_mpv(command: &mut process::Command) {
        use std::os::windows::process::CommandExt;
        use windows::Win32::System::Threading::CREATE_BREAKAWAY_FROM_JOB;
        command.creation_flags(CREATE_BREAKAWAY_FROM_JOB.0);
    }
}
