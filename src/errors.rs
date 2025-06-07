

use std::fmt;


pub type Result<T> = std::result::Result<T, Error>;


#[derive(Debug)]
pub enum Error {
    
    Io(std::io::Error),

    
    Json(serde_json::Error),

    
    Toml(toml::ser::Error),

    
    TomlDe(toml::de::Error),

    
    Config(String),

    
    Validation(String),

    
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


pub trait Context<T, E> {
    
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
