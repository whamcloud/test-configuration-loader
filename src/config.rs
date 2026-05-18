use crate::error::ConfigError;

/// Public API: the concrete `Config` type comes from `validate` module.
pub use crate::validate::Config;

/// Load configuration using the default search order and precedence:
/// defaults < file < environment
pub fn load() -> Result<Config, ConfigError> {
    load_with_explicit(None)
}

/// Load configuration but allow passing an explicit file path (high priority
/// over the default search locations).
pub fn load_with_explicit(path: Option<&str>) -> Result<Config, ConfigError> {
    let base = crate::defaults::defaults();
    let file_layer = crate::file::from_file(path)?;
    let env_layer = crate::env::from_env()?;

    // Merge order: base <- file <- env, where later layers win
    let merged = crate::merge::merge(base, file_layer);
    let merged = crate::merge::merge(merged, env_layer);

    crate::validate::from_partial(merged)
}
