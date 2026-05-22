//! Distributed node identity contracts.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable distributed node identifier.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodeId(String);

impl NodeId {
    /// Creates a node id.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the node id as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Role a node performs in a distributed `TuneX` cluster.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeRole {
    /// Cluster leader and control-plane owner.
    Leader,
    /// Generic worker node.
    Worker,
    /// Trainer execution node.
    Trainer,
    /// Dataset serving node.
    Dataset,
    /// Checkpoint persistence node.
    Checkpoint,
    /// Metrics aggregation node.
    Metrics,
    /// Read-only observer node.
    Observer,
}
