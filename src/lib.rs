//! Unified Configuration Loader
//!
//! This crate provides a strongly-typed configuration loader that merges
//! defaults, optional config files (TOML/YAML) and environment variables.
pub mod config;
pub mod defaults;
pub mod env;
pub mod error;
pub mod file;
pub mod merge;
pub mod partial;
pub mod validate;
pub mod types;

pub use config::Config;
