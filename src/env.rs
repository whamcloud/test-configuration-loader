use std::env;

use crate::error::ConfigError;
use crate::partial::{DatabasePartial, LoggingPartial, PartialConfig, ServerPartial};
use crate::types::LogLevel;

/// Read configuration from environment variables using the `DC_` prefix.
pub fn from_env() -> Result<PartialConfig, ConfigError> {
    let mut partial = PartialConfig::default();

    if let Ok(val) = env::var("DC_DATABASE_URL") {
        partial.database = Some(DatabasePartial { url: Some(val) });
    }

    if let Ok(val) = env::var("DC_PORT") {
        let parsed = val.parse::<u16>().map_err(|e| ConfigError::invalid_env("DC_PORT", val.clone(), e))?;
        partial.server.get_or_insert_with(ServerPartial::default).port = Some(parsed);
    }

    if let Ok(val) = env::var("DC_MAX_CONNECTIONS") {
        let parsed = val.parse::<u32>().map_err(|e| ConfigError::invalid_env("DC_MAX_CONNECTIONS", val.clone(), e))?;
        partial.server.get_or_insert_with(ServerPartial::default).max_connections = Some(parsed);
    }

    if let Ok(val) = env::var("DC_TIMEOUT_SECS") {
        let parsed = val.parse::<u64>().map_err(|e| ConfigError::invalid_env("DC_TIMEOUT_SECS", val.clone(), e))?;
        partial.server.get_or_insert_with(ServerPartial::default).timeout_secs = Some(parsed);
    }

    if let Ok(val) = env::var("DC_LOG_LEVEL") {
        let parsed = val.parse::<LogLevel>().map_err(|e| ConfigError::invalid_env("DC_LOG_LEVEL", val.clone(), e))?;
        partial.logging = Some(LoggingPartial { level: Some(parsed) });
    }

    if let Ok(val) = env::var("DC_CONFIG_FILE") {
        partial.config_file = Some(val);
    }

    Ok(partial)
}
