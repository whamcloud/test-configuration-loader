use crate::error::ConfigError;
use crate::partial::PartialConfig;
use crate::types::LogLevel;

#[derive(Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub max_connections: u32,
    pub timeout_secs: u64,
}

#[derive(Debug, Clone)]
pub struct LoggingConfig {
    pub level: LogLevel,
}

/// Validate and construct a `Config` from a `PartialConfig`.
pub fn from_partial(p: PartialConfig) -> Result<Config, ConfigError> {
    // Database: required
    let db = p.database.ok_or_else(|| ConfigError::MissingRequired {
        key: "database.url".to_string(),
        hint: "set DC_DATABASE_URL or provide in config file under [database].url".to_string(),
    })?;

    let url = db.url.ok_or_else(|| ConfigError::MissingRequired {
        key: "database.url".to_string(),
        hint: "set DC_DATABASE_URL or provide in config file under [database].url".to_string(),
    })?;

    if url.trim().is_empty() {
        return Err(ConfigError::Validation { field: "database.url".into(), message: "empty database URL".into() });
    }

    // Server: optional but must have sensible values
    let server_partial = p.server.ok_or_else(|| ConfigError::MissingRequired {
        key: "server".to_string(),
        hint: "server settings come from defaults, file, or DC_PORT/DC_MAX_CONNECTIONS".to_string(),
    })?;

    let port = server_partial.port.ok_or_else(|| ConfigError::MissingRequired { key: "server.port".into(), hint: "set DC_PORT or provide server.port in config file".into() })?;
    if port == 0 {
        return Err(ConfigError::Validation { field: "server.port".into(), message: "port must be > 0".into() });
    }

    let max_connections = server_partial.max_connections.unwrap_or(1);
    if max_connections == 0 {
        return Err(ConfigError::Validation { field: "server.max_connections".into(), message: "max_connections must be >= 1".into() });
    }

    let timeout_secs = server_partial.timeout_secs.unwrap_or(1);
    if timeout_secs == 0 {
        return Err(ConfigError::Validation { field: "server.timeout_secs".into(), message: "timeout_secs must be >= 1".into() });
    }

    let logging_partial = p.logging.ok_or_else(|| ConfigError::MissingRequired { key: "logging".into(), hint: "use defaults, file, or DC_LOG_LEVEL".into() })?;
    let level = logging_partial.level.unwrap_or(LogLevel::Info);

    Ok(Config {
        database: DatabaseConfig { url },
        server: ServerConfig { port, max_connections, timeout_secs },
        logging: LoggingConfig { level },
    })
}
