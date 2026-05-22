//! Configuration error types.

use thiserror::Error;

/// Errors returned while loading, overriding, or validating configuration.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ConfigError {
    /// JSON parsing failed.
    #[error("invalid json config: {0}")]
    Json(String),
    /// YAML parsing failed.
    #[error("invalid yaml config: {0}")]
    Yaml(String),
    /// TOML parsing failed.
    #[error("invalid toml config: {0}")]
    Toml(String),
    /// Config validation failed.
    #[error("invalid config: {0}")]
    Validation(String),
    /// Override path is unknown.
    #[error("unknown config override path: {0}")]
    UnknownOverride(String),
    /// Override value could not be parsed.
    #[error("invalid config override value for {path}: {value}")]
    InvalidOverride {
        /// Override path.
        path: String,
        /// Override value.
        value: String,
    },
}
