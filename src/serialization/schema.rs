//! Versioned schema contracts.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Semantic schema version for wire-compatible messages.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct SchemaVersion {
    /// Major schema version.
    pub major: u16,
    /// Minor schema version.
    pub minor: u16,
    /// Patch schema version.
    pub patch: u16,
}

impl SchemaVersion {
    /// Creates a schema version.
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns whether `self` can decode messages written with `writer`.
    #[must_use]
    pub const fn can_decode(self, writer: Self) -> bool {
        self.major == writer.major && self.minor >= writer.minor
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
