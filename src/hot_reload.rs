use crate::error::ConfigError;

#[cfg(feature = "hot-reload")]
pub fn watch<F: Fn(crate::validate::Config) + Send + Sync + 'static>(path: Option<&str>, on_update: F) -> Result<(), ConfigError> {
    use notify::{Config as NotifyConfig, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher};
    use std::sync::mpsc::channel;
    use std::thread;

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = RecommendedWatcher::new(tx, NotifyConfig::default())
        .map_err(|e| ConfigError::WatcherError(format!("failed to create watcher: {}", e)))?;

    // Determine a path to watch; if no explicit path, watch current directory
    let watch_path = path.map(|s| std::path::PathBuf::from(s)).unwrap_or_else(|| std::path::PathBuf::from("."));
    watcher.watch(&watch_path, RecursiveMode::NonRecursive).map_err(|e| ConfigError::WatcherError(format!("watch error: {}", e)))?;

    // Spawn a thread to handle events
    thread::spawn(move || {
        for res in rx.iter() {
            match res {
                Ok(_event) => {
                    // Try to reload; on success, call callback
                    if let Ok(cfg) = crate::config::load_with_explicit(path) {
                        on_update(cfg);
                    }
                }
                Err(_e) => {}
            }
        }
    });

    Ok(())
}

#[cfg(not(feature = "hot-reload"))]
pub fn watch<F: Fn(crate::validate::Config) + Send + Sync + 'static>(_path: Option<&str>, _on_update: F) -> Result<(), ConfigError> {
    Err(ConfigError::WatcherError("hot-reload feature is disabled".into()))
}
