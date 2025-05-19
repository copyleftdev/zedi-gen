//! Error types for zedi-gen

use std::fmt;

/// A specialized `Result` type for zedi-gen operations.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for zedi-gen operations.
#[derive(Debug)]
pub enum Error {
    /// I/O error
    Io(std::io::Error),

    /// JSON serialization/deserialization error
    Json(serde_json::Error),

    /// TOML serialization/deserialization error
    Toml(toml::ser::Error),

    /// TOML deserialization error
    TomlDe(toml::de::Error),

    /// Configuration error
    Config(String),

    /// Validation error
    Validation(String),

    /// Generation error
    Generation(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::Json(e) => write!(f, "JSON error: {}", e),
            Error::Toml(e) => write!(f, "TOML serialization error: {}", e),
            Error::TomlDe(e) => write!(f, "TOML deserialization error: {}", e),
            Error::Config(msg) => write!(f, "Configuration error: {}", msg),
            Error::Validation(msg) => write!(f, "Validation error: {}", msg),
            Error::Generation(msg) => write!(f, "Generation error: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(ref e) => Some(e),
            Error::Json(ref e) => Some(e),
            Error::Toml(ref e) => Some(e),
            Error::TomlDe(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::Toml(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::TomlDe(err)
    }
}

/// Helper trait for adding context to errors
pub trait Context<T, E> {
    /// Add context to an error
    fn context<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static;
}

impl<T, E> Context<T, E> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|e| Error::Config(format!("{}: {}", context, e)))
    }
}
