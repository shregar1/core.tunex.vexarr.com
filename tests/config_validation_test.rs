use tunex_core::config::{
    ConfigFormat, ConfigOverride, OverrideSource, TunexConfig, ValidateConfig,
};

const CONFIG_YAML: &str = r#"
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
"#;

#[test]
fn yaml_config_loads_and_validates() {
    let config =
        TunexConfig::from_str(CONFIG_YAML, ConfigFormat::Yaml).expect("yaml config should parse");

    assert!(config.validate().is_valid());
    assert_eq!(config.dataset.plugin, "tunex-dataset-jsonl");
    assert_eq!(config.runtime.workers.tokenizer, 8);
    assert_eq!(config.trainer.batch_size, 4);
}

#[test]
fn cli_overrides_participate_in_validation() {
    let mut config = TunexConfig::default();

    config
        .apply_cli_overrides(&[
            ConfigOverride::new("dataset.plugin", "tunex-dataset-jsonl", OverrideSource::Cli),
            ConfigOverride::new("dataset.path", "./data/train.jsonl", OverrideSource::Cli),
            ConfigOverride::new(
                "tokenizer.plugin",
                "tunex-tokenizer-hf",
                OverrideSource::Cli,
            ),
            ConfigOverride::new("tokenizer.model", "mock-tokenizer", OverrideSource::Cli),
            ConfigOverride::new("trainer.backend", "mock", OverrideSource::Cli),
        ])
        .expect("supported overrides should apply");

    assert!(config.validate().is_valid());
}

#[test]
fn default_config_reports_missing_required_module_fields() {
    let report = TunexConfig::default().validate();

    assert!(!report.is_valid());
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.path == "dataset.plugin"));
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.path == "tokenizer.model"));
}
