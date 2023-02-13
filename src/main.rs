use std::process::{Command, self};
use ff2mpv_rust::{get_mpv_message, send_browser_message};

fn main() {
    let message = match get_mpv_message() {
        Ok(msg) => msg,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(-1)
        }
    };

    let mpv = Command::new("mpv")
                    .arg(message.url)
                    .spawn();

    if let Err(e) = mpv {
        eprintln!("{}", e);
        process::exit(-1);
    }

    if let Err(e) = send_browser_message("ok") {
        eprintln!("{}", e);
        process::exit(-1);
    }
}

