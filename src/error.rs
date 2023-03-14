use std::io;
use std::fmt;
use std::fmt::Display;

pub enum FF2MpvError {
    NoConfig,
    IOError(io::Error),
    JSONError(serde_json::Error),
}

impl From<io::Error> for FF2MpvError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<serde_json::Error> for FF2MpvError {
    fn from(value: serde_json::Error) -> Self {
        Self::JSONError(value)
    }
}

impl Display for FF2MpvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoConfig => write!(f, "Config doesn't exist"),
            Self::IOError(e) => write!(f, "IO Error: {e}"),
            Self::JSONError(e) => write!(f, "JSON Error: {e}"),
        }
    }
}
