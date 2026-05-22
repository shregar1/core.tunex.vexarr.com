//! Batch-level data contracts.

use serde::{Deserialize, Serialize};

/// Packed token batch ready to be passed to a trainer backend.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PackedBatch {
    /// Contiguous packed token ids.
    pub token_ids: Vec<u32>,
    /// Sequence start offsets inside `token_ids`.
    pub sequence_offsets: Vec<u32>,
    /// Per-token labels. Use `-100` for ignored positions.
    pub labels: Vec<i32>,
}

impl PackedBatch {
    /// Creates a packed batch with explicit labels.
    #[must_use]
    pub const fn new(token_ids: Vec<u32>, sequence_offsets: Vec<u32>, labels: Vec<i32>) -> Self {
        Self {
            token_ids,
            sequence_offsets,
            labels,
        }
    }

    /// Returns the number of packed tokens.
    #[must_use]
    pub fn token_count(&self) -> usize {
        self.token_ids.len()
    }

    /// Returns the number of packed sequences.
    #[must_use]
    pub fn sequence_count(&self) -> usize {
        self.sequence_offsets.len()
    }

    /// Returns true when no tokens are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.token_ids.is_empty()
    }
}
