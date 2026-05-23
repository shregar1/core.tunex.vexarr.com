//! Test assertion contracts.

use serde::{Deserialize, Serialize};

/// Severity of a test assertion.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AssertionLevel {
    /// Required assertion.
    Required,
    /// Advisory assertion.
    Advisory,
}

/// Result of one contract assertion.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TestAssertion {
    /// Assertion name.
    pub name: String,
    /// Assertion level.
    pub level: AssertionLevel,
    /// Whether the assertion passed.
    pub passed: bool,
    /// Human-readable assertion message.
    pub message: String,
}

impl TestAssertion {
    /// Creates a passing assertion.
    #[must_use]
    pub fn passed(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            level: AssertionLevel::Required,
            passed: true,
            message: message.into(),
        }
    }

    /// Creates a failing assertion.
    #[must_use]
    pub fn failed(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            level: AssertionLevel::Required,
            passed: false,
            message: message.into(),
        }
    }

    /// Sets assertion level.
    #[must_use]
    pub const fn with_level(mut self, level: AssertionLevel) -> Self {
        self.level = level;
        self
    }
}
