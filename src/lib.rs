mod config;
mod error;

use config::Config;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;