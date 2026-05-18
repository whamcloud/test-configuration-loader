use crate::partial::{DatabasePartial, LoggingPartial, PartialConfig, ServerPartial};

/// Merge two `PartialConfig` values where `overlay` takes precedence over
/// `base`. `None` in `overlay` does not clear `Some` in `base`.
pub fn merge(base: PartialConfig, overlay: PartialConfig) -> PartialConfig {
    // Database
    let database = match (overlay.database, base.database) {
        (Some(o), Some(b)) => Some(DatabasePartial { url: o.url.or(b.url) }),
        (Some(o), None) => Some(o),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    };

    // Server
    let server = match (overlay.server, base.server) {
        (Some(o), Some(b)) => Some(ServerPartial {
            port: o.port.or(b.port),
            max_connections: o.max_connections.or(b.max_connections),
            timeout_secs: o.timeout_secs.or(b.timeout_secs),
        }),
        (Some(o), None) => Some(o),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    };

    // Logging
    let logging = match (overlay.logging, base.logging) {
        (Some(o), Some(b)) => Some(LoggingPartial { level: o.level.or(b.level) }),
        (Some(o), None) => Some(o),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    };

    let config_file = overlay.config_file.or(base.config_file);

    PartialConfig { database, server, logging, config_file }
}
