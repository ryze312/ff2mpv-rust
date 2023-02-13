use std::io::{self, Read, Write};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MpvMessage {
    pub url: String
}

pub fn get_mpv_message() -> Result<MpvMessage, String> {
    let message = match get_browser_message() {
        Ok(msg) => msg,
        Err(e) => return Err(format!("IO Error: {}", e))
    };

    match serde_json::from_str(&message) {
        Ok(msg) => Ok(msg),
        Err(e) => Err(format!("JSON Error: {}", e))
    }
}

pub fn get_browser_message() -> io::Result<String> {
    let mut stdin = io::stdin();
    let mut buf: [u8; 4] = [0; 4];
    stdin.read_exact(&mut buf)?;

    let length = u32::from_ne_bytes(buf);
    let mut reader = io::BufReader::new(stdin.take(length as u64));
    
    let mut string = String::with_capacity(length as usize);
    reader.read_to_string(&mut string)?;
    Ok(string)
}

pub fn send_browser_message(message: &str) -> io::Result<()> {
    let length = (message.len() as u32).to_ne_bytes();
    let message = message.as_bytes();
    
    let mut stdout = io::stdout();
    stdout.write(&length)?;
    stdout.write(&message)?;
    Ok(())
}
