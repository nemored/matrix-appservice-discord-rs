use std::{fs::File, io::{Read, self}, path::Path, fmt::Display};

use serde::Deserialize;

#[derive(Debug)]
enum ErrorKind {
    IoError(io::Error),
    DeserializeError(toml::de::Error),
}

#[derive(Debug)]
struct ConfigError {
    kind: ErrorKind,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ErrorKind::*;
        match &self.kind {
            IoError(error) => write!(f, "IO error: {}", error),
            DeserializeError(error) => write!(f, "TOML deserialization error: {}", error),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<io::Error> for ConfigError {
    fn from(value: io::Error) -> Self {
        Self { kind: ErrorKind::IoError(value) }
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(value: toml::de::Error) -> Self {
        Self { kind: ErrorKind::DeserializeError(value) }
    }
}

type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Deserialize)]
pub(crate) struct Config {
    homeserver_url: String,
    server_name: String,
}

impl Config {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Config> {
        let mut buf = String::new();
        
        File::open(path)?.read_to_string(&mut buf)?;
        
        Ok(toml::from_str(&buf)?)
    }
}