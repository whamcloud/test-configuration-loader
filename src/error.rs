use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file '{path}': {source}")]
    ReadFile {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse config file '{path}': {source}")]
    ParseFile {
        path: String,
        #[source]
        source: toml::de::Error,
    },

    #[error("invalid environment variable '{key}': {message}")]
    InvalidEnv { key: String, message: String },

    #[error("missing required configuration: {field}")]
    MissingField { field: String },

    #[error("validation error: {message}")]
    Validation { message: String },
}
