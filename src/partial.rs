use crate::types::LogLevel;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PartialConfig {
    pub database: Option<DatabasePartial>,
    pub server: Option<ServerPartial>,
    pub logging: Option<LoggingPartial>,
    /// Optional explicit path hinted via file contents or env
    pub config_file: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct DatabasePartial {
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ServerPartial {
    pub port: Option<u16>,
    pub max_connections: Option<u32>,
    pub timeout_secs: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LoggingPartial {
    pub level: Option<LogLevel>,
}
