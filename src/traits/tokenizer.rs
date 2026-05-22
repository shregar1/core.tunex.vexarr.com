//! Tokenizer contract.

use async_trait::async_trait;

use crate::{
    types::{RawSample, TokenizedSample},
    TunexResult,
};

/// Converts raw samples into tokenized samples.
#[async_trait]
pub trait Tokenizer: Send + Sync {
    /// Tokenizes one raw sample.
    ///
    /// # Errors
    ///
    /// Returns an error when the sample cannot be tokenized.
    async fn tokenize(&self, sample: &RawSample) -> TunexResult<TokenizedSample>;
}
