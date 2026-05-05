use std::env;
use std::process;

use ff2mpv_rust::{command::Command, config::Config};
use tracing::{debug, error};

fn main() {
    let config = Config::build();
    util::setup_logging(&config.log_level);
    debug!("Loaded config {config:?}");

    let mut args = env::args();
    args.next(); // Skip binary path

    let command_name = args.next().unwrap_or_default();
    let command = get_command(&command_name);

    if let Err(e) = command.execute(&config) {
        error!("Execution failed: {e}");
        process::exit(-1);
    }
}

fn get_command(name: &str) -> Command {
    match name {
        "help" => Command::ShowHelp,
        "manifest" => Command::ShowManifest,
        "manifest_chromium" => Command::ShowManifestChromium,
        "validate" => Command::ValidateConfig,
        _ => Command::FF2Mpv,
    }
}

mod util {
    use std::str::FromStr;

    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    #[cfg(unix)]
    const FALLBACK_LOGFILE_NAME: &str = "ff2mpv-rust.log";
    #[cfg(unix)]
    const FALLBACK_LOGFILE_LOC: &str = "~/.var/log/";

    fn get_common_layers(log_level: &str) -> LevelFilter {
        let level = if log_level.is_empty() {
            "info"
        } else {
            log_level
        };

        LevelFilter::from_str(level).unwrap_or(LevelFilter::INFO)
    }

    #[cfg(target_os = "linux")]
    pub(super) fn setup_logging(log_level: &str) {
        use std::fs;

        let log_level_filter = self::get_common_layers(log_level);

        match tracing_journald::layer() {
            Ok(journald_layer) => tracing_subscriber::registry()
                .with(log_level_filter)
                .with(journald_layer)
                .init(),
            Err(err) => {
                let fallback_file = format!("{FALLBACK_LOGFILE_LOC}{FALLBACK_LOGFILE_NAME}");
                eprintln!("Failed to connect to journald {err:?}\nFalling back to {fallback_file}");

                if fs::exists(FALLBACK_LOGFILE_LOC).is_err() {
                    fs::create_dir_all(FALLBACK_LOGFILE_LOC)
                        .expect("failed to create dir for log file");
                }

                tracing_subscriber::registry()
                    .with(log_level_filter)
                    .with(
                        tracing_subscriber::fmt::layer()
                            .with_ansi(false) // Disable colors
                            .with_writer(
                                fs::File::open(&fallback_file).expect("Failed to create log file"),
                            ),
                    )
                    .init();
            }
        }
    }

    // TODO: Remove `unstable_windows_logging` feature flag once testing on windows is validated
    #[cfg(all(target_os = "windows", feature = "unstable_windows_logging"))]
    pub(super) fn setup_logging(log_level: &str) {
        let log_level_filter = self::get_common_layers(log_level);

        tracing_subscriber::registry()
            .with(log_level_filter)
            .with(
                cfg!(target_os = "windows")
                    .then(|| tracing_layer_win_eventlog::EventLogLayer::new("hello_world")),
            )
            .init();
    }

    #[cfg(all(target_os = "windows", not(feature = "unstable_windows_logging")))]
    pub(super) fn setup_logging(_log_level: &str) {
        // No-op
    }
}
