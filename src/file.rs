use std::fs;
use std::path::Path;

use crate::error::ConfigError;
use crate::partial::PartialConfig;

/// Try to locate and parse a configuration file. If `explicit` is `Some(path)`
/// the function will return an error when the file does not exist. If `None`,
/// the file layer is optional and missing files are ignored.
pub fn from_file(explicit: Option<&str>) -> Result<PartialConfig, ConfigError> {
    let candidate = if let Some(path) = explicit {
        Some(path.to_string())
    } else if let Ok(env_path) = std::env::var("DC_CONFIG_FILE") {
        Some(env_path)
    } else {
        // Common defaults
        let candidates = ["config.toml", "config.yaml", "config.yml"];
        candidates.iter().find(|p| Path::new(p).exists()).map(|s| s.to_string())
    };

    let path = match candidate {
        Some(p) => p,
        None => return Ok(PartialConfig::default()),
    };

    let path_buf = Path::new(&path).to_path_buf();
    let s = fs::read_to_string(&path).map_err(|e| {
        if explicit.is_some() {
            ConfigError::FileNotFound(path_buf.clone())
        } else {
            ConfigError::Io(e)
        }
    })?;

    // Detect by extension
    if path.ends_with(".toml") {
        toml::from_str::<PartialConfig>(&s).map_err(|e| ConfigError::ParseFile {
            path: path_buf,
            source: Box::new(e),
        })
    } else if path.ends_with(".yaml") || path.ends_with(".yml") {
        serde_yaml::from_str::<PartialConfig>(&s).map_err(|e| ConfigError::ParseFile {
            path: path_buf,
            source: Box::new(e),
        })
    } else {
        Err(ConfigError::UnsupportedFormat(path_buf
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string()))
    }
}
