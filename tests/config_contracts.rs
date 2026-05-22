use tunex_core::config::{
    ConfigFormat, ConfigOverride, OverrideSource, PackingStrategy, TunexConfig, ValidateConfig,
};

const YAML_CONFIG: &str = r#"
version: "0.1"
dataset:
  plugin: "tunex-dataset-jsonl"
  path: "./data/train.jsonl"
tokenizer:
  plugin: "tunex-tokenizer-hf"
  model: "meta-llama/Llama-3.1-8B"
packer:
  max_seq_len: 4096
  packing_strategy: "best_fit"
runtime:
  enabled: true
  workers:
    dataset: 4
    tokenizer: 8
    packer: 4
    trainer: 1
    checkpoint: 1
  queues:
    dataset_to_tokenizer: 1024
    tokenizer_to_packer: 512
    packer_to_trainer: 128
    trainer_to_checkpoint: 16
trainer:
  backend: "unsloth"
  lora_rank: 16
  batch_size: 4
checkpoint:
  interval_steps: 100
metrics:
  exporters:
    - "tracing"
"#;

#[test]
fn parses_user_target_yaml_config() {
    let config = TunexConfig::from_str(YAML_CONFIG, ConfigFormat::Yaml).unwrap();

    assert_eq!(config.version, "0.1");
    assert_eq!(config.dataset.plugin, "tunex-dataset-jsonl");
    assert_eq!(config.tokenizer.model, "meta-llama/Llama-3.1-8B");
    assert_eq!(config.packer.packing_strategy, PackingStrategy::BestFit);
    assert_eq!(config.runtime.workers.tokenizer, 8);
    assert_eq!(config.trainer.backend, "unsloth");
    assert!(config.validate().is_valid());
}

#[test]
fn parses_json_and_toml_configs() {
    let json = r#"{
      "version": "0.1",
      "dataset": {"plugin": "jsonl", "path": "train.jsonl"},
      "tokenizer": {"plugin": "hf", "model": "tok"},
      "trainer": {"backend": "unsloth"}
    }"#;
    let toml = r#"
version = "0.1"

[dataset]
plugin = "jsonl"
path = "train.jsonl"

[tokenizer]
plugin = "hf"
model = "tok"

[trainer]
backend = "unsloth"
"#;

    assert!(TunexConfig::from_str(json, ConfigFormat::Json)
        .unwrap()
        .validate()
        .is_valid());
    assert!(TunexConfig::from_str(toml, ConfigFormat::Toml)
        .unwrap()
        .validate()
        .is_valid());
}

#[test]
fn applies_cli_overrides_to_typed_fields() {
    let mut config = TunexConfig::default();
    config
        .apply_cli_overrides(&[
            ConfigOverride::new("dataset.plugin", "jsonl", OverrideSource::Cli),
            ConfigOverride::new("dataset.path", "train.jsonl", OverrideSource::Cli),
            ConfigOverride::new("tokenizer.plugin", "hf", OverrideSource::Cli),
            ConfigOverride::new("tokenizer.model", "tok", OverrideSource::Cli),
            ConfigOverride::new("trainer.backend", "unsloth", OverrideSource::Cli),
            ConfigOverride::new("trainer.batch_size", "8", OverrideSource::Cli),
            ConfigOverride::new("runtime.workers.tokenizer", "12", OverrideSource::Cli),
        ])
        .unwrap();

    assert_eq!(config.trainer.batch_size, 8);
    assert_eq!(config.runtime.workers.tokenizer, 12);
    assert!(config.validate().is_valid());
}

#[test]
fn validation_reports_missing_required_enabled_module_fields() {
    let config = TunexConfig::default();
    let report = config.validate();

    assert!(!report.is_valid());
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.path == "dataset.plugin"));
    assert!(config.validate_or_error().is_err());
}

#[test]
fn generates_json_schema_for_versioned_config() {
    let schema = TunexConfig::json_schema();
    let schema_json = serde_json::to_value(schema).unwrap();

    assert_eq!(schema_json["title"], "TunexConfig");
}
