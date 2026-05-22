//! Versioned configuration schema, parsing, overrides, and validation.

pub mod checkpoint;
pub mod dataset;
pub mod distributed;
pub mod error;
pub mod metrics;
pub mod packer;
pub mod root;
pub mod runtime;
pub mod tokenizer;
pub mod trainer;
pub mod validation;

pub use checkpoint::CheckpointConfig;
pub use dataset::DatasetConfig;
pub use distributed::DistributedConfig;
pub use error::ConfigError;
pub use metrics::MetricsConfig;
pub use packer::{PackerConfig, PackingStrategy};
pub use root::{ConfigFormat, ConfigOverride, OverrideSource, PluginConfig, TunexConfig};
pub use runtime::{QueueConfig, RuntimeConfig, WorkerConfig};
pub use tokenizer::TokenizerConfig;
pub use trainer::TrainerConfig;
pub use validation::{ConfigValidationIssue, ConfigValidationReport, ValidateConfig};
