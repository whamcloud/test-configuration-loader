use crate::partial::{DatabasePartial, LoggingPartial, PartialConfig, ServerPartial};
use crate::types::LogLevel;

/// Returns a `PartialConfig` populated with safe defaults where appropriate.
pub fn defaults() -> PartialConfig {
    PartialConfig {
        database: Some(DatabasePartial { url: None }),
        server: Some(ServerPartial {
            port: Some(8080),
            max_connections: Some(100),
            timeout_secs: Some(30),
        }),
        logging: Some(LoggingPartial {
            level: Some(LogLevel::Info),
        }),
        config_file: None,
    }
}
