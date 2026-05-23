//! Stable public API metadata and compatibility contracts.

pub mod capabilities;
pub mod compatibility;
pub mod version;

pub use capabilities::{CapabilitySet, FeatureFlag};
pub use compatibility::{CompatibilityLevel, Stability};
pub use version::{ApiVersion, CoreVersion};
