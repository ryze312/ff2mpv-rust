use std::env;

use ff2mpv_rust::{command::Command, error::FF2MpvError};

fn main() -> Result<(), FF2MpvError> {
    let mut args = env::args();
    args.next(); // Skip binary path

    let command_name = args.next().unwrap_or_default();
    let command = get_command(&command_name);
    command.execute()
}

fn get_command(name: &str) -> Command {
    match name {
        "help" => Command::ShowHelp,
        "manifest" => Command::ShowManifest,
        "validate" => Command::ValidateConfig,
        _ => Command::FF2Mpv,
    }
}
