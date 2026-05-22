//! Sample-level data contracts.

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

/// Inline metadata optimized for the common small-record path.
pub type SampleMetadata = SmallVec<[(String, String); 4]>;

/// Raw input sample emitted by a dataset source.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RawSample {
    /// Optional stable source identifier.
    pub id: Option<String>,
    /// Raw sample payload.
    pub payload: Bytes,
    /// Small metadata collection for source-specific annotations.
    pub metadata: SampleMetadata,
}

impl RawSample {
    /// Creates a raw sample from shared bytes.
    #[must_use]
    pub fn new(payload: Bytes) -> Self {
        Self {
            id: None,
            payload,
            metadata: SampleMetadata::new(),
        }
    }

    /// Creates a raw sample with a stable identifier.
    #[must_use]
    pub fn with_id(id: impl Into<String>, payload: Bytes) -> Self {
        Self {
            id: Some(id.into()),
            payload,
            metadata: SampleMetadata::new(),
        }
    }
}

/// Tokenized representation of a single sample.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TokenizedSample {
    /// Optional stable source identifier.
    pub id: Option<String>,
    /// Token ids produced by a tokenizer.
    pub token_ids: Vec<u32>,
    /// Attention mask aligned to `token_ids`.
    pub attention_mask: Vec<u8>,
}

impl TokenizedSample {
    /// Creates a tokenized sample without an identifier.
    #[must_use]
    pub fn new(token_ids: Vec<u32>) -> Self {
        let attention_mask = vec![1; token_ids.len()];
        Self {
            id: None,
            token_ids,
            attention_mask,
        }
    }
}
