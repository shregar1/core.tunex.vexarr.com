use std::{error::Error, fmt};

use tunex_core::error::{
    Diagnostic, DiagnosticLevel, ErrorCode, ErrorContext, ErrorKind, Recoverability, TunexError,
    TunexResult,
};

#[derive(Debug)]
struct SourceFailure;

impl fmt::Display for SourceFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("source failed")
    }
}

impl Error for SourceFailure {}

#[test]
fn error_carries_code_recoverability_context_diagnostic_and_metadata() {
    let context = ErrorContext::new()
        .with_module("tokenizer")
        .with_stage("tokenize")
        .with_worker("worker-7")
        .with_batch("batch-9")
        .with_trace("trace-abc")
        .with_metadata("plugin", "hf");
    let diagnostic = Diagnostic::new(DiagnosticLevel::Error, "tokenizer failed")
        .with_hint("check tokenizer model path");
    let error = TunexError::new(ErrorKind::Tokenizer, "model not found")
        .with_code(ErrorCode::new("TUNEX-TOKENIZER-0001"))
        .with_recoverability(Recoverability::Retryable)
        .with_context(context)
        .with_diagnostic(diagnostic)
        .with_metadata("model", "llama")
        .with_source(SourceFailure);

    assert_eq!(error.kind(), ErrorKind::Tokenizer);
    assert_eq!(error.category(), "tokenizer");
    assert_eq!(error.code().as_str(), "TUNEX-TOKENIZER-0001");
    assert_eq!(error.recoverability(), Recoverability::Retryable);
    assert_eq!(error.context().module_name.as_deref(), Some("tokenizer"));
    assert_eq!(error.context().stage_name.as_deref(), Some("tokenize"));
    assert_eq!(error.context().worker_id.as_deref(), Some("worker-7"));
    assert_eq!(error.context().batch_id.as_deref(), Some("batch-9"));
    assert_eq!(error.context().trace_id.as_deref(), Some("trace-abc"));
    assert_eq!(error.context().metadata["plugin"], "hf");
    assert_eq!(error.metadata()["model"], "llama");
    assert_eq!(
        error.diagnostic().unwrap().hint.as_deref(),
        Some("check tokenizer model path")
    );
    assert_eq!(error.source().unwrap().to_string(), "source failed");
}

#[test]
fn all_error_kinds_have_stable_default_codes() {
    let kinds = [
        ErrorKind::Config,
        ErrorKind::Plugin,
        ErrorKind::Dataset,
        ErrorKind::Tokenizer,
        ErrorKind::Packer,
        ErrorKind::Runtime,
        ErrorKind::Trainer,
        ErrorKind::Checkpoint,
        ErrorKind::Distributed,
        ErrorKind::Metrics,
        ErrorKind::Io,
        ErrorKind::Serialization,
        ErrorKind::Memory,
        ErrorKind::Cancelled,
        ErrorKind::Timeout,
        ErrorKind::Unsupported,
        ErrorKind::Internal,
    ];

    for kind in kinds {
        let error = TunexError::new(kind, "failure");
        assert!(error.code().as_str().starts_with("TUNEX-"));
        assert!(error.code().as_str().ends_with("-0000"));
        assert_eq!(error.category(), kind.as_str());
    }
}

#[test]
fn tunex_result_alias_uses_unified_error() {
    fn fail() -> TunexResult<()> {
        Err(TunexError::new(ErrorKind::Cancelled, "job cancelled").recoverable())
    }

    let error = fail().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Cancelled);
    assert_eq!(error.recoverability(), Recoverability::Recoverable);
}
