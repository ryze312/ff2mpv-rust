use std::env;
use std::process;

use ff2mpv_rust::command::Command;

fn main() {
    let mut args = env::args();
    args.next(); // Skip binary path

    let command_name = args.next().unwrap_or_default();
    let command = get_command(&command_name);
    if let Err(e) = command.execute() {
        eprintln!("{e}");
        process::exit(-1);
    }
}

fn get_command(name: &str) -> Command {
    match name {
        "help" => Command::ShowHelp,
        "manifest" => Command::ShowManifest,
        "validate" => Command::ValidateConfig,
        _ => Command::FF2Mpv,
    }
}
