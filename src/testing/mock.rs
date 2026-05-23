//! Mock module implementations for contract tests.

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;

use crate::{
    checkpoint::{CheckpointMetadata, CheckpointSnapshot, CheckpointWriter},
    traits::{DatasetSource, SequencePacker, Tokenizer, TrainerBackend},
    types::{PackedBatch, RawSample, StepMetrics, TokenizedSample},
    TunexResult,
};

/// Mock dataset source backed by an in-memory queue.
#[derive(Clone, Debug, Default)]
pub struct MockDatasetSource {
    samples: Arc<Mutex<VecDeque<RawSample>>>,
}

impl MockDatasetSource {
    /// Creates a mock dataset source.
    #[must_use]
    pub fn new(samples: impl IntoIterator<Item = RawSample>) -> Self {
        Self {
            samples: Arc::new(Mutex::new(samples.into_iter().collect())),
        }
    }

    /// Returns remaining sample count.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.samples.lock().map_or(0, |samples| samples.len())
    }
}

#[async_trait]
impl DatasetSource for MockDatasetSource {
    async fn next_sample(&self) -> TunexResult<Option<RawSample>> {
        Ok(self.samples.lock().expect("mock dataset mutex").pop_front())
    }
}

/// Mock tokenizer that maps bytes directly to token ids.
#[derive(Clone, Copy, Debug, Default)]
pub struct MockTokenizer;

#[async_trait]
impl Tokenizer for MockTokenizer {
    async fn tokenize(&self, sample: &RawSample) -> TunexResult<TokenizedSample> {
        let mut tokenized =
            TokenizedSample::new(sample.payload.iter().map(|byte| u32::from(*byte)).collect());
        tokenized.id.clone_from(&sample.id);
        Ok(tokenized)
    }
}

/// Mock packer that concatenates tokenized samples.
#[derive(Clone, Copy, Debug, Default)]
pub struct MockSequencePacker;

#[async_trait]
impl SequencePacker for MockSequencePacker {
    async fn pack(&self, samples: &[TokenizedSample]) -> TunexResult<PackedBatch> {
        let mut token_ids = Vec::new();
        let mut sequence_offsets = Vec::with_capacity(samples.len());
        for sample in samples {
            sequence_offsets.push(u32::try_from(token_ids.len()).unwrap_or(u32::MAX));
            token_ids.extend(sample.token_ids.iter().copied());
        }
        let labels = token_ids
            .iter()
            .map(|token| i32::try_from(*token).unwrap_or(i32::MAX))
            .collect();
        Ok(PackedBatch::new(token_ids, sequence_offsets, labels))
    }
}

/// Mock trainer backend that returns deterministic step metrics.
#[derive(Clone, Debug, Default)]
pub struct MockTrainerBackend {
    next_step: Arc<Mutex<u64>>,
}

impl MockTrainerBackend {
    /// Creates a mock trainer backend.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl TrainerBackend for MockTrainerBackend {
    async fn train_step(&self, batch: &PackedBatch) -> TunexResult<StepMetrics> {
        let mut step = self.next_step.lock().expect("mock trainer mutex");
        let metrics = StepMetrics::new(
            *step,
            u64::try_from(batch.token_count()).unwrap_or(u64::MAX),
            u64::try_from(batch.sequence_count()).unwrap_or(u64::MAX),
            1,
        );
        *step = step.saturating_add(1);
        Ok(metrics)
    }
}

/// Mock checkpoint writer that records writes in memory.
#[derive(Clone, Debug, Default)]
pub struct MockCheckpointWriter {
    metadata: Arc<Mutex<Vec<CheckpointMetadata>>>,
    snapshots: Arc<Mutex<Vec<CheckpointSnapshot>>>,
}

impl MockCheckpointWriter {
    /// Returns written checkpoint metadata.
    #[must_use]
    pub fn metadata(&self) -> Vec<CheckpointMetadata> {
        self.metadata
            .lock()
            .map_or_else(|_| Vec::new(), |items| items.clone())
    }

    /// Returns written checkpoint snapshots.
    #[must_use]
    pub fn snapshots(&self) -> Vec<CheckpointSnapshot> {
        self.snapshots
            .lock()
            .map_or_else(|_| Vec::new(), |items| items.clone())
    }
}

#[async_trait]
impl CheckpointWriter for MockCheckpointWriter {
    async fn write_snapshot(
        &self,
        snapshot: CheckpointSnapshot,
    ) -> TunexResult<CheckpointMetadata> {
        let metadata = snapshot.metadata.clone();
        self.snapshots
            .lock()
            .expect("mock checkpoint mutex")
            .push(snapshot);
        self.metadata
            .lock()
            .expect("mock checkpoint mutex")
            .push(metadata.clone());
        Ok(metadata)
    }

    async fn write_metadata(&self, metadata: CheckpointMetadata) -> TunexResult<()> {
        self.metadata
            .lock()
            .expect("mock checkpoint mutex")
            .push(metadata);
        Ok(())
    }
}
