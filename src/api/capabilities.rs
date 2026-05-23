//! Module capability discovery contracts.

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use super::{ApiVersion, Stability};

/// Feature flag advertised or required by an API surface.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FeatureFlag {
    /// Tokio-backed orchestration.
    Tokio,
    /// Rayon-backed CPU parallelism.
    Rayon,
    /// Distributed coordination APIs.
    Distributed,
    /// Metrics APIs.
    Metrics,
    /// WASM plugin APIs.
    Wasm,
    /// CUDA backend integration APIs.
    Cuda,
    /// Metal backend integration APIs.
    Metal,
    /// `ROCm` backend integration APIs.
    Rocm,
    /// Custom feature flag.
    Custom(String),
}

/// Capability set exposed by a module or plugin.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CapabilitySet {
    /// Module or crate name.
    pub module: String,
    /// API version implemented by this module.
    pub api_version: ApiVersion,
    /// Stability metadata for this capability set.
    pub stability: Stability,
    /// Feature flags exposed or required by this module.
    pub features: SmallVec<[FeatureFlag; 8]>,
}

impl CapabilitySet {
    /// Creates an empty capability set.
    #[must_use]
    pub fn new(module: impl Into<String>, api_version: ApiVersion, stability: Stability) -> Self {
        Self {
            module: module.into(),
            api_version,
            stability,
            features: SmallVec::new(),
        }
    }

    /// Adds one feature flag.
    #[must_use]
    pub fn with_feature(mut self, feature: FeatureFlag) -> Self {
        if !self.features.contains(&feature) {
            self.features.push(feature);
        }
        self
    }

    /// Returns whether the capability set declares a feature.
    #[must_use]
    pub fn supports(&self, feature: &FeatureFlag) -> bool {
        self.features.contains(feature)
    }

    /// Returns whether this capability set can satisfy a required API version.
    #[must_use]
    pub fn is_compatible_with(&self, required: ApiVersion) -> bool {
        self.api_version.is_compatible_with(required)
            && !matches!(self.stability.level, super::CompatibilityLevel::Internal)
    }
}
