//! Checkpoint snapshot contracts.

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use super::CheckpointMetadata;
use crate::memory::{MemoryHint, ZeroCopyBuffer};

/// Snapshot payload associated with checkpoint metadata.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckpointSnapshot {
    /// Checkpoint metadata.
    pub metadata: CheckpointMetadata,
    /// Zero-copy snapshot chunks.
    pub chunks: SmallVec<[Bytes; 4]>,
    /// Whether compression is requested or applied.
    pub compressed: bool,
}

impl CheckpointSnapshot {
    /// Creates an empty checkpoint snapshot.
    #[must_use]
    pub fn new(metadata: CheckpointMetadata) -> Self {
        Self {
            metadata,
            chunks: SmallVec::new(),
            compressed: false,
        }
    }

    /// Adds a snapshot chunk.
    #[must_use]
    pub fn with_chunk(mut self, chunk: Bytes) -> Self {
        self.chunks.push(chunk);
        self
    }

    /// Marks the snapshot as compressed.
    #[must_use]
    pub const fn compressed(mut self) -> Self {
        self.compressed = true;
        self
    }

    /// Returns the total byte size of all chunks.
    #[must_use]
    pub fn total_bytes(&self) -> usize {
        self.chunks.iter().map(Bytes::len).sum()
    }

    /// Returns chunks as zero-copy buffers.
    #[must_use]
    pub fn buffers(&self) -> SmallVec<[ZeroCopyBuffer; 4]> {
        self.chunks
            .iter()
            .cloned()
            .map(|chunk| ZeroCopyBuffer::with_hint(chunk, MemoryHint::Snapshot))
            .collect()
    }
}
