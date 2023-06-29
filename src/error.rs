use std::{error, fmt::Display, io};

use matrix_sdk_appservice::matrix_sdk;

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
    MatrixAppService(matrix_sdk_appservice::Error),
    MatrixIdParse(matrix_sdk::IdParseError),
    SerdeYaml(serde_yaml::Error),
    UrlParse(url::ParseError),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ErrorKind::*;
        match &self.kind {
            Io(error) => write!(f, "IO error: {}", error),
            MatrixAppService(error) => write!(f, "AppService error: {}", error),
            MatrixIdParse(error) => write!(f, "Matrix ID parsing error: {}", error),
            SerdeYaml(error) => write!(f, "Serde YAML error: {}", error),
            UrlParse(error) => write!(f, "URL parse error: {}", error),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self { 
            kind: ErrorKind::Io(value)
        }
    }
}

impl From<matrix_sdk::IdParseError> for Error {
    fn from(value: matrix_sdk::IdParseError) -> Self {
        Self {
            kind: ErrorKind::MatrixIdParse(value),
        }
    }
}

impl From<matrix_sdk_appservice::Error> for Error {
    fn from(value: matrix_sdk_appservice::Error) -> Self {
        Self {
            kind: ErrorKind::MatrixAppService(value),
        }
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(value: serde_yaml::Error) -> Self {
        Self { 
            kind: ErrorKind::SerdeYaml(value)
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self {
            kind: ErrorKind::UrlParse(value),
        }
    }
}
