//! Runtime execution contracts.

pub mod context;
pub mod engine;
pub mod executor;
pub mod handle;
pub mod state;
pub mod stats;

pub use context::{RuntimeContext, RuntimeOptions};
pub use engine::RuntimeEngine;
pub use executor::RuntimeExecutor;
pub use handle::RuntimeHandle;
pub use state::{RuntimeLayer, RuntimeMode, RuntimeState, ShutdownReason, WorkerSignal};
pub use stats::RuntimeStats;

pub use crate::cancellation::{CancellationReason, CancellationToken, DrainPolicy, ShutdownMode};
