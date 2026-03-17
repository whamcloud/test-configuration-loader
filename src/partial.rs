use crate::config::{Config, DatabaseConfig, ServerConfig};
use crate::error::ConfigError;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PartialConfig {
    pub server: Option<PartialServerConfig>,
    pub database: Option<PartialDatabaseConfig>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PartialServerConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PartialDatabaseConfig {
    pub url: Option<String>,
    pub pool_size: Option<u32>,
}

impl PartialConfig {
    pub fn defaults() -> Self {
        Self {
            server: Some(PartialServerConfig {
                host: Some("127.0.0.1".to_string()),
                port: Some(8080),
            }),
            database: Some(PartialDatabaseConfig {
                url: None,
                pool_size: Some(10),
            }),
        }
    }

    pub fn merge(self, other: Self) -> Self {
        Self {
            server: merge_server(self.server, other.server),
            database: merge_db(self.database, other.database),
        }
    }

    pub fn from_env(env: &HashMap<String, String>) -> Result<Self, ConfigError> {
        let mut server = PartialServerConfig::default();
        let mut db = PartialDatabaseConfig::default();

        if let Some(port) = env.get("APP_SERVER_PORT") {
            server.port = Some(port.parse().map_err(|_| ConfigError::InvalidEnv {
                key: "APP_SERVER_PORT".into(),
                message: "must be a valid u16".into(),
            })?);
        }

        if let Some(host) = env.get("APP_SERVER_HOST") {
            server.host = Some(host.clone());
        }

        if let Some(url) = env.get("APP_DATABASE_URL") {
            db.url = Some(url.clone());
        }

        if let Some(size) = env.get("APP_DATABASE_POOL_SIZE") {
            db.pool_size = Some(size.parse().map_err(|_| ConfigError::InvalidEnv {
                key: "APP_DATABASE_POOL_SIZE".into(),
                message: "must be a valid u32".into(),
            })?);
        }

        Ok(Self {
            server: Some(server),
            database: Some(db),
        })
    }

    pub fn into_config(self) -> Result<Config, ConfigError> {
        let server = self.server.ok_or(ConfigError::MissingField {
            field: "server".into(),
        })?;

        let database = self.database.ok_or(ConfigError::MissingField {
            field: "database".into(),
        })?;

        let host = server.host.ok_or(ConfigError::MissingField {
            field: "server.host".into(),
        })?;

        let port = server.port.ok_or(ConfigError::MissingField {
            field: "server.port".into(),
        })?;

        let url = database.url.ok_or(ConfigError::MissingField {
            field: "database.url".into(),
        })?;

        let pool_size = database.pool_size.ok_or(ConfigError::MissingField {
            field: "database.pool_size".into(),
        })?;

        if port == 0 {
            return Err(ConfigError::Validation {
                message: "server.port must be > 0".into(),
            });
        }

        if pool_size == 0 {
            return Err(ConfigError::Validation {
                message: "database.pool_size must be > 0".into(),
            });
        }

        Ok(Config {
            server: ServerConfig { host, port },
            database: DatabaseConfig { url, pool_size },
        })
    }
}

fn merge_server(
    base: Option<PartialServerConfig>,
    other: Option<PartialServerConfig>,
) -> Option<PartialServerConfig> {
    match (base, other) {
        (Some(b), Some(o)) => Some(PartialServerConfig {
            host: o.host.or(b.host),
            port: o.port.or(b.port),
        }),
        (None, Some(o)) => Some(o),
        (Some(b), None) => Some(b),
        (None, None) => None,
    }
}

fn merge_db(
    base: Option<PartialDatabaseConfig>,
    other: Option<PartialDatabaseConfig>,
) -> Option<PartialDatabaseConfig> {
    match (base, other) {
        (Some(b), Some(o)) => Some(PartialDatabaseConfig {
            url: o.url.or(b.url),
            pool_size: o.pool_size.or(b.pool_size),
        }),
        (None, Some(o)) => Some(o),
        (Some(b), None) => Some(b),
        (None, None) => None,
    }
}
