//! Structured error context.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Machine-readable context attached to an error.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ErrorContext {
    /// Module reporting the error.
    pub module_name: Option<String>,
    /// Pipeline stage reporting the error.
    pub stage_name: Option<String>,
    /// Worker identifier.
    pub worker_id: Option<String>,
    /// Batch identifier.
    pub batch_id: Option<String>,
    /// Trace identifier.
    pub trace_id: Option<String>,
    /// Additional context metadata.
    pub metadata: BTreeMap<String, String>,
}

impl ErrorContext {
    /// Creates an empty error context.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the module name.
    #[must_use]
    pub fn with_module(mut self, module_name: impl Into<String>) -> Self {
        self.module_name = Some(module_name.into());
        self
    }

    /// Sets the stage name.
    #[must_use]
    pub fn with_stage(mut self, stage_name: impl Into<String>) -> Self {
        self.stage_name = Some(stage_name.into());
        self
    }

    /// Sets the worker id.
    #[must_use]
    pub fn with_worker(mut self, worker_id: impl Into<String>) -> Self {
        self.worker_id = Some(worker_id.into());
        self
    }

    /// Sets the batch id.
    #[must_use]
    pub fn with_batch(mut self, batch_id: impl Into<String>) -> Self {
        self.batch_id = Some(batch_id.into());
        self
    }

    /// Sets the trace id.
    #[must_use]
    pub fn with_trace(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    /// Adds one context metadata entry.
    #[must_use]
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}
