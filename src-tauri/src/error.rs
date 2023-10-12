use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TsugiError {
    message: String,
    source: Option<Box<dyn Error>>,
}

impl fmt::Display for TsugiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TsugiError: {}", self.message)
    }
}

impl Error for TsugiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl Serialize for TsugiError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("TsugiError", 1)?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}

impl From<std::io::Error> for TsugiError {
    fn from(e: std::io::Error) -> Self {
        TsugiError {
            message: e.to_string(),
            source: Some(Box::new(e)),
        }
    }
}

impl From<reqwest::Error> for TsugiError {
    fn from(e: reqwest::Error) -> Self {
        TsugiError {
            message: e.to_string(),
            source: Some(Box::new(e)),
        }
    }
}

impl From<serde_json::Error> for TsugiError {
    fn from(e: serde_json::Error) -> Self {
        TsugiError {
            message: e.to_string(),
            source: Some(Box::new(e)),
        }
    }
}

impl From<rusqlite::Error> for TsugiError {
    fn from(e: rusqlite::Error) -> Self {
        TsugiError {
            message: e.to_string(),
            source: Some(Box::new(e)),
        }
    }
}
