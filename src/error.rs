use std::path::PathBuf;

use thiserror::Error;

/// Errors produced by the configuration loader.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("configuration file not found: {0}")]
    FileNotFound(PathBuf),

    #[error("failed to parse configuration file {path}: {source}")]
    ParseFile {
        path: PathBuf,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("invalid environment variable `{name}`: `{value}` — {source}")]
    InvalidEnvVar {
        name: String,
        value: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("missing required configuration key `{key}` — {hint}")]
    MissingRequired { key: String, hint: String },

    #[error("validation failed for `{field}`: {message}")]
    Validation { field: String, message: String },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("unsupported configuration file format: {0}")]
    UnsupportedFormat(String),

    #[error("hot-reload watcher error: {0}")]
    WatcherError(String),
}

impl ConfigError {
    pub(crate) fn invalid_env<S1, S2, E>(name: S1, value: S2, e: E) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        E: std::error::Error + Send + Sync + 'static,
    {
        ConfigError::InvalidEnvVar {
            name: name.into(),
            value: value.into(),
            source: Box::new(e),
        }
    }
}
