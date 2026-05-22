//! Configuration validation contracts.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// One validation issue.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct ConfigValidationIssue {
    /// Dot-separated config path.
    pub path: String,
    /// Human-readable validation message.
    pub message: String,
}

impl ConfigValidationIssue {
    /// Creates a validation issue.
    #[must_use]
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}

/// Config validation report.
#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct ConfigValidationReport {
    /// Validation issues.
    pub issues: Vec<ConfigValidationIssue>,
}

impl ConfigValidationReport {
    /// Returns true when no validation issues were found.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }

    /// Adds one validation issue.
    pub fn push(&mut self, path: impl Into<String>, message: impl Into<String>) {
        self.issues.push(ConfigValidationIssue::new(path, message));
    }
}

/// Validates a config object.
pub trait ValidateConfig {
    /// Returns a validation report.
    fn validate(&self) -> ConfigValidationReport;
}
