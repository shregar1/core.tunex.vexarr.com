//! Public API version contracts.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Version of a public API contract.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ApiVersion {
    /// Major API version.
    pub major: u16,
    /// Minor API version.
    pub minor: u16,
    /// Patch API version.
    pub patch: u16,
}

impl ApiVersion {
    /// Creates an API version.
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns whether this version can satisfy a required API version.
    #[must_use]
    pub const fn is_compatible_with(self, required: Self) -> bool {
        self.major == required.major && self.minor >= required.minor
    }
}

impl Default for ApiVersion {
    fn default() -> Self {
        Self::new(0, 1, 0)
    }
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Version of the `tunex-core` crate providing an API contract.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CoreVersion {
    /// Package version.
    pub version: String,
    /// API version.
    pub api_version: ApiVersion,
}

impl CoreVersion {
    /// Creates core version metadata.
    #[must_use]
    pub fn new(version: impl Into<String>, api_version: ApiVersion) -> Self {
        Self {
            version: version.into(),
            api_version,
        }
    }

    /// Returns metadata for the currently compiled crate.
    #[must_use]
    pub fn current() -> Self {
        Self::new(env!("CARGO_PKG_VERSION"), ApiVersion::default())
    }
}
