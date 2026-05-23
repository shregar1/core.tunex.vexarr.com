//! Public API compatibility and stability metadata.

use serde::{Deserialize, Serialize};

use super::ApiVersion;

/// Public API compatibility level.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityLevel {
    /// Stable public API.
    Stable,
    /// Experimental public API behind feature gates.
    Experimental,
    /// Deprecated public API retained for compatibility.
    Deprecated,
    /// Internal API not intended for downstream modules.
    Internal,
}

/// Stability metadata for a public capability or API surface.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Stability {
    /// Compatibility level.
    pub level: CompatibilityLevel,
    /// API version where the surface became available.
    pub since: ApiVersion,
    /// Optional deprecation version.
    pub deprecated_since: Option<ApiVersion>,
    /// Optional human-readable deprecation replacement.
    pub replacement: Option<String>,
    /// Optional feature gate required to use this surface.
    pub feature_gate: Option<String>,
}

impl Stability {
    /// Creates stable metadata.
    #[must_use]
    pub const fn stable(since: ApiVersion) -> Self {
        Self {
            level: CompatibilityLevel::Stable,
            since,
            deprecated_since: None,
            replacement: None,
            feature_gate: None,
        }
    }

    /// Creates experimental metadata.
    #[must_use]
    pub fn experimental(since: ApiVersion, feature_gate: impl Into<String>) -> Self {
        Self {
            level: CompatibilityLevel::Experimental,
            since,
            deprecated_since: None,
            replacement: None,
            feature_gate: Some(feature_gate.into()),
        }
    }

    /// Marks metadata as deprecated.
    #[must_use]
    pub fn deprecated(mut self, since: ApiVersion, replacement: impl Into<String>) -> Self {
        self.level = CompatibilityLevel::Deprecated;
        self.deprecated_since = Some(since);
        self.replacement = Some(replacement.into());
        self
    }

    /// Returns whether the API is available without experimental feature gates.
    #[must_use]
    pub const fn is_public_stable(&self) -> bool {
        matches!(self.level, CompatibilityLevel::Stable)
    }
}
