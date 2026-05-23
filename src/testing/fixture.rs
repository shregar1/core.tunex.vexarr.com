//! Shared test fixture contracts.

use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::{
    config::TunexConfig,
    types::{RawSample, TokenizedSample},
};

/// Reusable fixture for module contract tests.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TestFixture {
    /// Fixture name.
    pub name: String,
    /// Configuration under test.
    pub config: TunexConfig,
    /// Raw samples used by dataset and tokenizer tests.
    pub raw_samples: Vec<RawSample>,
    /// Tokenized samples used by packer tests.
    pub tokenized_samples: Vec<TokenizedSample>,
}

impl TestFixture {
    /// Creates an empty fixture.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            config: TunexConfig::default(),
            raw_samples: Vec::new(),
            tokenized_samples: Vec::new(),
        }
    }

    /// Adds a raw sample.
    #[must_use]
    pub fn with_raw_sample(mut self, sample: RawSample) -> Self {
        self.raw_samples.push(sample);
        self
    }

    /// Adds a UTF-8 raw sample.
    #[must_use]
    pub fn with_text_sample(self, id: impl Into<String>, text: impl Into<String>) -> Self {
        self.with_raw_sample(RawSample::with_id(id, Bytes::from(text.into())))
    }

    /// Adds a tokenized sample.
    #[must_use]
    pub fn with_tokenized_sample(mut self, sample: TokenizedSample) -> Self {
        self.tokenized_samples.push(sample);
        self
    }

    /// Sets configuration.
    #[must_use]
    pub fn with_config(mut self, config: TunexConfig) -> Self {
        self.config = config;
        self
    }
}
