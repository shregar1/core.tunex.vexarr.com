use tunex_core::prelude::*;

const YAML: &str = r#"
version: "0.1"
dataset:
  plugin: "tunex-dataset-jsonl"
  path: "./data/train.jsonl"
tokenizer:
  plugin: "tunex-tokenizer-hf"
  model: "meta-llama/Llama-3.1-8B"
trainer:
  backend: "mock"
  batch_size: 4
"#;

fn main() -> TunexResult<()> {
    let mut config = TunexConfig::from_str(YAML, ConfigFormat::Yaml)?;
    config.apply_cli_overrides(&[ConfigOverride::new(
        "runtime.workers.tokenizer",
        "8",
        OverrideSource::Cli,
    )])?;

    let report = config.validate();
    if !report.is_valid() {
        return Err(TunexError::config(format!(
            "config has {} validation issue(s)",
            report.issues.len()
        )));
    }

    println!(
        "loaded dataset plugin {} with tokenizer workers {}",
        config.dataset.plugin, config.runtime.workers.tokenizer
    );
    Ok(())
}
