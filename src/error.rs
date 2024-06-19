pub type Result<T> = std::result::Result<T, Error>;

use std::fmt::Display;

use serde::{de, ser};

/// Shaft Error type.
#[derive(Debug)]
pub struct Error(String);

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
