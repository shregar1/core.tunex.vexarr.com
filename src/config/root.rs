//! Root configuration schema and loading helpers.

use std::{collections::BTreeMap, env, str::FromStr};

use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

use super::{
    CheckpointConfig, ConfigError, ConfigValidationReport, DatasetConfig, DistributedConfig,
    MetricsConfig, PackerConfig, RuntimeConfig, TokenizerConfig, TrainerConfig, ValidateConfig,
};
use crate::{TunexError, TunexResult};

/// Supported config serialization formats.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigFormat {
    /// JSON config.
    Json,
    /// YAML config.
    Yaml,
    /// TOML config.
    Toml,
}

/// Source for a config override.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OverrideSource {
    /// Environment variable override.
    Environment,
    /// Command-line override.
    Cli,
    /// Runtime override.
    Runtime,
}

/// Dot-path config override.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct ConfigOverride {
    /// Dot-separated config path.
    pub path: String,
    /// Override value as text.
    pub value: String,
    /// Override source.
    pub source: OverrideSource,
}

impl ConfigOverride {
    /// Creates a config override.
    #[must_use]
    pub fn new(path: impl Into<String>, value: impl Into<String>, source: OverrideSource) -> Self {
        Self {
            path: path.into(),
            value: value.into(),
            source,
        }
    }
}

/// Plugin selection config.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct PluginConfig {
    /// Whether this plugin entry is enabled.
    pub enabled: bool,
    /// Plugin id or name.
    pub plugin: String,
    /// Optional minimum compatible version.
    pub version: Option<String>,
    /// Plugin-specific settings.
    pub settings: BTreeMap<String, String>,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            plugin: String::new(),
            version: None,
            settings: BTreeMap::new(),
        }
    }
}

/// Root versioned `TuneX` configuration.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct TunexConfig {
    /// Config schema version.
    pub version: String,
    /// Dataset config.
    pub dataset: DatasetConfig,
    /// Tokenizer config.
    pub tokenizer: TokenizerConfig,
    /// Packer config.
    pub packer: PackerConfig,
    /// Runtime config.
    pub runtime: RuntimeConfig,
    /// Trainer config.
    pub trainer: TrainerConfig,
    /// Checkpoint config.
    pub checkpoint: CheckpointConfig,
    /// Metrics config.
    pub metrics: MetricsConfig,
    /// Distributed config.
    pub distributed: DistributedConfig,
    /// Additional plugin selections.
    #[serde(default)]
    pub plugins: Vec<PluginConfig>,
}

impl TunexConfig {
    /// Current supported config schema version.
    pub const CURRENT_VERSION: &'static str = "0.1";

    /// Parses config from a supported format.
    ///
    /// # Errors
    ///
    /// Returns an error when parsing fails.
    pub fn from_str(input: &str, format: ConfigFormat) -> TunexResult<Self> {
        let config = match format {
            ConfigFormat::Json => serde_json::from_str(input)
                .map_err(|error| config_error(&ConfigError::Json(error.to_string())))?,
            ConfigFormat::Yaml => serde_yaml::from_str(input)
                .map_err(|error| config_error(&ConfigError::Yaml(error.to_string())))?,
            ConfigFormat::Toml => toml::from_str(input)
                .map_err(|error| config_error(&ConfigError::Toml(error.to_string())))?,
        };

        Ok(config)
    }

    /// Generates a JSON schema for the config.
    #[must_use]
    pub fn json_schema() -> schemars::Schema {
        schema_for!(Self)
    }

    /// Applies dot-path CLI overrides.
    ///
    /// # Errors
    ///
    /// Returns an error when an override path is unknown or a value cannot be parsed.
    pub fn apply_cli_overrides(&mut self, overrides: &[ConfigOverride]) -> TunexResult<()> {
        for override_value in overrides {
            self.apply_override(override_value)?;
        }
        Ok(())
    }

    /// Applies environment overrides with the provided prefix.
    ///
    /// Environment names are converted from `PREFIX_DATASET_PLUGIN` to `dataset.plugin`.
    ///
    /// # Errors
    ///
    /// Returns an error when an override path is unknown or a value cannot be parsed.
    pub fn apply_env_overrides(&mut self, prefix: &str) -> TunexResult<()> {
        let prefix = format!("{prefix}_");
        for (key, value) in env::vars() {
            let Some(path) = key.strip_prefix(&prefix) else {
                continue;
            };
            let path = path.to_ascii_lowercase().replace('_', ".");
            self.apply_override(&ConfigOverride::new(
                path,
                value,
                OverrideSource::Environment,
            ))?;
        }
        Ok(())
    }

    /// Applies one config override.
    ///
    /// # Errors
    ///
    /// Returns an error when the override path is unknown or a value cannot be parsed.
    pub fn apply_override(&mut self, override_value: &ConfigOverride) -> TunexResult<()> {
        let path = override_value.path.as_str();
        let value = override_value.value.as_str();

        match path {
            "version" => value.clone_into(&mut self.version),
            "dataset.enabled" => self.dataset.enabled = parse_bool(path, value)?,
            "dataset.plugin" => value.clone_into(&mut self.dataset.plugin),
            "dataset.path" => value.clone_into(&mut self.dataset.path),
            "tokenizer.enabled" => self.tokenizer.enabled = parse_bool(path, value)?,
            "tokenizer.plugin" => value.clone_into(&mut self.tokenizer.plugin),
            "tokenizer.model" => value.clone_into(&mut self.tokenizer.model),
            "tokenizer.max_length" => {
                self.tokenizer.max_length = parse_optional_usize(path, value)?;
            }
            "packer.enabled" => self.packer.enabled = parse_bool(path, value)?,
            "packer.plugin" => self.packer.plugin = parse_optional_string(value),
            "packer.max_seq_len" => self.packer.max_seq_len = parse_usize(path, value)?,
            "runtime.enabled" => self.runtime.enabled = parse_bool(path, value)?,
            "runtime.workers.dataset" => self.runtime.workers.dataset = parse_usize(path, value)?,
            "runtime.workers.tokenizer" => {
                self.runtime.workers.tokenizer = parse_usize(path, value)?;
            }
            "runtime.workers.packer" => self.runtime.workers.packer = parse_usize(path, value)?,
            "runtime.workers.trainer" => self.runtime.workers.trainer = parse_usize(path, value)?,
            "runtime.workers.checkpoint" => {
                self.runtime.workers.checkpoint = parse_usize(path, value)?;
            }
            "runtime.queues.dataset_to_tokenizer" => {
                self.runtime.queues.dataset_to_tokenizer = parse_usize(path, value)?;
            }
            "runtime.queues.tokenizer_to_packer" => {
                self.runtime.queues.tokenizer_to_packer = parse_usize(path, value)?;
            }
            "runtime.queues.packer_to_trainer" => {
                self.runtime.queues.packer_to_trainer = parse_usize(path, value)?;
            }
            "runtime.queues.trainer_to_checkpoint" => {
                self.runtime.queues.trainer_to_checkpoint = parse_usize(path, value)?;
            }
            "trainer.enabled" => self.trainer.enabled = parse_bool(path, value)?,
            "trainer.backend" => value.clone_into(&mut self.trainer.backend),
            "trainer.plugin" => self.trainer.plugin = parse_optional_string(value),
            "trainer.lora_rank" => self.trainer.lora_rank = parse_optional_usize(path, value)?,
            "trainer.batch_size" => self.trainer.batch_size = parse_usize(path, value)?,
            "checkpoint.enabled" => self.checkpoint.enabled = parse_bool(path, value)?,
            "checkpoint.plugin" => self.checkpoint.plugin = parse_optional_string(value),
            "checkpoint.destination" => value.clone_into(&mut self.checkpoint.destination),
            "checkpoint.interval_steps" => {
                self.checkpoint.interval_steps = parse_usize(path, value)?;
            }
            "metrics.enabled" => self.metrics.enabled = parse_bool(path, value)?,
            "metrics.plugin" => self.metrics.plugin = parse_optional_string(value),
            "metrics.namespace" => value.clone_into(&mut self.metrics.namespace),
            "distributed.enabled" => self.distributed.enabled = parse_bool(path, value)?,
            "distributed.plugin" => self.distributed.plugin = parse_optional_string(value),
            "distributed.endpoint" => self.distributed.endpoint = parse_optional_string(value),
            _ => {
                return Err(config_error(&ConfigError::UnknownOverride(
                    override_value.path.clone(),
                )));
            }
        }

        Ok(())
    }

    /// Validates and returns an error if issues are present.
    ///
    /// # Errors
    ///
    /// Returns an error when validation issues exist.
    pub fn validate_or_error(&self) -> TunexResult<()> {
        let report = self.validate();
        if report.is_valid() {
            return Ok(());
        }

        let joined = report
            .issues
            .iter()
            .map(|issue| format!("{}: {}", issue.path, issue.message))
            .collect::<Vec<_>>()
            .join("; ");
        Err(config_error(&ConfigError::Validation(joined)))
    }
}

impl Default for TunexConfig {
    fn default() -> Self {
        Self {
            version: Self::CURRENT_VERSION.to_owned(),
            dataset: DatasetConfig::default(),
            tokenizer: TokenizerConfig::default(),
            packer: PackerConfig::default(),
            runtime: RuntimeConfig::default(),
            trainer: TrainerConfig::default(),
            checkpoint: CheckpointConfig::default(),
            metrics: MetricsConfig::default(),
            distributed: DistributedConfig::default(),
            plugins: Vec::new(),
        }
    }
}

impl ValidateConfig for TunexConfig {
    fn validate(&self) -> ConfigValidationReport {
        let mut report = ConfigValidationReport::default();

        if self.version != Self::CURRENT_VERSION {
            report.push("version", "unsupported config version");
        }
        if self.dataset.enabled && self.dataset.plugin.is_empty() {
            report.push("dataset.plugin", "enabled dataset requires a plugin");
        }
        if self.dataset.enabled && self.dataset.path.is_empty() {
            report.push("dataset.path", "enabled dataset requires a path");
        }
        if self.tokenizer.enabled && self.tokenizer.plugin.is_empty() {
            report.push("tokenizer.plugin", "enabled tokenizer requires a plugin");
        }
        if self.tokenizer.enabled && self.tokenizer.model.is_empty() {
            report.push("tokenizer.model", "enabled tokenizer requires a model");
        }
        if self.packer.enabled && self.packer.max_seq_len == 0 {
            report.push(
                "packer.max_seq_len",
                "max sequence length must be greater than zero",
            );
        }
        if self.trainer.enabled && self.trainer.backend.is_empty() {
            report.push("trainer.backend", "enabled trainer requires a backend");
        }
        if self.trainer.enabled && self.trainer.batch_size == 0 {
            report.push("trainer.batch_size", "batch size must be greater than zero");
        }
        if self.checkpoint.enabled && self.checkpoint.interval_steps == 0 {
            report.push(
                "checkpoint.interval_steps",
                "checkpoint interval must be greater than zero",
            );
        }
        if self.metrics.enabled && self.metrics.namespace.is_empty() {
            report.push("metrics.namespace", "metrics namespace cannot be empty");
        }
        validate_worker_count(
            &mut report,
            "runtime.workers.dataset",
            self.runtime.workers.dataset,
        );
        validate_worker_count(
            &mut report,
            "runtime.workers.tokenizer",
            self.runtime.workers.tokenizer,
        );
        validate_worker_count(
            &mut report,
            "runtime.workers.packer",
            self.runtime.workers.packer,
        );
        validate_worker_count(
            &mut report,
            "runtime.workers.trainer",
            self.runtime.workers.trainer,
        );
        validate_queue_size(
            &mut report,
            "runtime.queues.dataset_to_tokenizer",
            self.runtime.queues.dataset_to_tokenizer,
        );
        validate_queue_size(
            &mut report,
            "runtime.queues.tokenizer_to_packer",
            self.runtime.queues.tokenizer_to_packer,
        );
        validate_queue_size(
            &mut report,
            "runtime.queues.packer_to_trainer",
            self.runtime.queues.packer_to_trainer,
        );

        report
    }
}

fn validate_worker_count(report: &mut ConfigValidationReport, path: &str, value: usize) {
    if value == 0 {
        report.push(path, "worker count must be greater than zero");
    }
}

fn validate_queue_size(report: &mut ConfigValidationReport, path: &str, value: usize) {
    if value == 0 {
        report.push(path, "queue size must be greater than zero");
    }
}

fn parse_bool(path: &str, value: &str) -> TunexResult<bool> {
    bool::from_str(value).map_err(|_| invalid_override(path, value))
}

fn parse_usize(path: &str, value: &str) -> TunexResult<usize> {
    usize::from_str(value).map_err(|_| invalid_override(path, value))
}

fn parse_optional_usize(path: &str, value: &str) -> TunexResult<Option<usize>> {
    if value.is_empty() || value.eq_ignore_ascii_case("none") {
        return Ok(None);
    }
    parse_usize(path, value).map(Some)
}

fn parse_optional_string(value: &str) -> Option<String> {
    if value.is_empty() || value.eq_ignore_ascii_case("none") {
        None
    } else {
        Some(value.to_owned())
    }
}

fn invalid_override(path: &str, value: &str) -> TunexError {
    config_error(&ConfigError::InvalidOverride {
        path: path.to_owned(),
        value: value.to_owned(),
    })
}

fn config_error(error: &ConfigError) -> TunexError {
    TunexError::Config(error.to_string())
}
