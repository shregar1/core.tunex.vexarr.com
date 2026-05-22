//! Plugin version contract.

use std::{cmp::Ordering, fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use super::error::PluginError;

/// Semantic plugin version descriptor.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PluginVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl PluginVersion {
    /// Creates a plugin version from numeric components.
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version.
    #[must_use]
    pub const fn major(&self) -> u64 {
        self.major
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(&self) -> u64 {
        self.minor
    }

    /// Returns the patch version.
    #[must_use]
    pub const fn patch(&self) -> u64 {
        self.patch
    }

    /// Returns true when `self` is compatible with a required version.
    #[must_use]
    pub const fn is_compatible_with(&self, required: &Self) -> bool {
        self.major == required.major
            && (self.minor > required.minor
                || (self.minor == required.minor && self.patch >= required.patch))
    }
}

impl fmt::Display for PluginVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for PluginVersion {
    type Err = PluginError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut parts = value.split('.');
        let major = parse_part(parts.next(), value)?;
        let minor = parse_part(parts.next(), value)?;
        let patch = parse_part(parts.next(), value)?;

        if parts.next().is_some() {
            return Err(PluginError::InvalidVersion(value.to_owned()));
        }

        Ok(Self::new(major, minor, patch))
    }
}

impl PartialOrd for PluginVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PluginVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch))
    }
}

fn parse_part(part: Option<&str>, original: &str) -> Result<u64, PluginError> {
    part.ok_or_else(|| PluginError::InvalidVersion(original.to_owned()))?
        .parse()
        .map_err(|_| PluginError::InvalidVersion(original.to_owned()))
}
