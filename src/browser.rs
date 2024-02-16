use std::io::{self, Read, Write};

use serde::Deserialize;

use crate::error::FF2MpvError;

#[derive(Deserialize)]
pub struct FF2MpvMessage {
    pub url: String,
    pub options: Vec<String>,
}

pub fn send_reply() -> Result<(), io::Error> {
    // "ok" formatted as a JSON string
    send_message("\"ok\"")
}

pub fn get_mpv_message() -> Result<FF2MpvMessage, FF2MpvError> {
    let message = read_message()?;
    let ff2mpv_message = serde_json::from_str(&message)?;
    Ok(ff2mpv_message)
}

fn read_message() -> Result<String, io::Error> {
    let mut stdin = io::stdin().lock();

    let mut len = 0_u32.to_ne_bytes();
    stdin.read_exact(&mut len)?;
    let len = u32::from_ne_bytes(len);

    let mut reader = stdin.take(len as u64);
    let mut msg = String::with_capacity(len as usize);
    reader.read_to_string(&mut msg)?;
    Ok(msg)
}

fn send_message(message: &str) -> Result<(), io::Error> {
    let length = (message.len() as u32).to_ne_bytes();
    let message = message.as_bytes();

    let mut stdout = io::stdout().lock();
    stdout.write_all(&length)?;
    stdout.write_all(message)?;
    Ok(())
}
