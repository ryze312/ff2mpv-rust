use serde::Deserialize;
use std::io;
use std::io::BufReader;
use std::io::{Read, Write};

use crate::error::FF2MpvError;

#[derive(Deserialize)]
pub struct FF2MpvMessage {
    pub url: String,
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
    let mut stdin = io::stdin();
    let mut buf: [u8; 4] = [0; 4];
    stdin.read_exact(&mut buf)?;

    let length = u32::from_ne_bytes(buf);
    let mut reader = BufReader::new(stdin.take(length as u64));

    let mut string = String::with_capacity(length as usize);
    reader.read_to_string(&mut string)?;
    Ok(string)
}

fn send_message(message: &str) -> Result<(), io::Error> {
    let length = (message.len() as u32).to_ne_bytes();
    let message = message.as_bytes();

    let mut stdout = io::stdout();
    stdout.write_all(&length)?;
    stdout.write_all(message)?;
    Ok(())
}
