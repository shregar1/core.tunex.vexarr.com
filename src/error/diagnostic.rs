//! Human-readable diagnostics attached to errors.

use serde::{Deserialize, Serialize};

/// Diagnostic severity.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticLevel {
    /// Informational diagnostic.
    Info,
    /// Warning diagnostic.
    Warning,
    /// Error diagnostic.
    Error,
}

/// Human-readable diagnostic information.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Diagnostic {
    /// Diagnostic level.
    pub level: DiagnosticLevel,
    /// Main diagnostic message.
    pub message: String,
    /// Optional remediation hint.
    pub hint: Option<String>,
}

impl Diagnostic {
    /// Creates a diagnostic.
    #[must_use]
    pub fn new(level: DiagnosticLevel, message: impl Into<String>) -> Self {
        Self {
            level,
            message: message.into(),
            hint: None,
        }
    }

    /// Adds a remediation hint.
    #[must_use]
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}
