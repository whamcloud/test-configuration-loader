use crate::error::ConfigError;
use crate::partial::PartialConfig;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let _ = dotenvy::dotenv();

        let env: HashMap<String, String> = std::env::vars().collect();
        let file_contents = std::fs::read_to_string("config.toml").ok();

        Self::load_from_sources(file_contents.as_deref(), &env)
    }

    pub fn load_from_sources(
        file_contents: Option<&str>,
        env: &HashMap<String, String>,
    ) -> Result<Self, ConfigError> {
        let defaults = PartialConfig::defaults();

        let file_cfg = if let Some(contents) = file_contents {
            toml::from_str::<PartialConfig>(contents).map_err(|e| ConfigError::ParseFile {
                path: "config.toml".into(),
                source: e,
            })?
        } else {
            PartialConfig::default()
        };

        let env_cfg = PartialConfig::from_env(env)?;

        let merged = defaults.merge(file_cfg).merge(env_cfg);

        merged.into_config()
    }
}
