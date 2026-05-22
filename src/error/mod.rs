//! Unified error and diagnostics model for `TuneX` core.

pub mod context;
pub mod diagnostic;
pub mod kind;
pub mod result;

pub use context::ErrorContext;
pub use diagnostic::{Diagnostic, DiagnosticLevel};
pub use kind::{ErrorCode, ErrorKind, Recoverability};
pub use result::TunexResult;

use std::{collections::BTreeMap, error::Error, fmt};

/// Top-level error type for `TuneX` core.
#[derive(Debug)]
pub struct TunexError {
    inner: Box<TunexErrorInner>,
}

#[derive(Debug)]
struct TunexErrorInner {
    kind: ErrorKind,
    code: ErrorCode,
    message: String,
    recoverability: Recoverability,
    context: ErrorContext,
    diagnostic: Option<Diagnostic>,
    metadata: BTreeMap<String, String>,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl TunexError {
    /// Creates an error with the default stable code for its kind.
    #[must_use]
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            inner: Box::new(TunexErrorInner {
                code: kind.default_code(),
                kind,
                message: message.into(),
                recoverability: Recoverability::Fatal,
                context: ErrorContext::default(),
                diagnostic: None,
                metadata: BTreeMap::new(),
                source: None,
            }),
        }
    }

    /// Creates a config error.
    #[must_use]
    pub fn config(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Config, message)
    }

    /// Creates a plugin error.
    #[must_use]
    pub fn plugin(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Plugin, message)
    }

    /// Creates a topology/internal orchestration error.
    #[must_use]
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Internal, message)
    }

    /// Returns the error kind.
    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        self.inner.kind
    }

    /// Returns the stable error code.
    #[must_use]
    pub fn code(&self) -> &ErrorCode {
        &self.inner.code
    }

    /// Returns the human-readable error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.inner.message
    }

    /// Returns recoverability classification.
    #[must_use]
    pub fn recoverability(&self) -> Recoverability {
        self.inner.recoverability
    }

    /// Returns the structured context.
    #[must_use]
    pub fn context(&self) -> &ErrorContext {
        &self.inner.context
    }

    /// Returns optional human-readable diagnostics.
    #[must_use]
    pub fn diagnostic(&self) -> Option<&Diagnostic> {
        self.inner.diagnostic.as_ref()
    }

    /// Returns machine-readable metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.inner.metadata
    }

    /// Returns the stable category name for this error.
    #[must_use]
    pub fn category(&self) -> &'static str {
        self.inner.kind.as_str()
    }

    /// Replaces the stable error code.
    #[must_use]
    pub fn with_code(mut self, code: ErrorCode) -> Self {
        self.inner.code = code;
        self
    }

    /// Replaces recoverability.
    #[must_use]
    pub fn with_recoverability(mut self, recoverability: Recoverability) -> Self {
        self.inner.recoverability = recoverability;
        self
    }

    /// Marks the error recoverable.
    #[must_use]
    pub fn recoverable(self) -> Self {
        self.with_recoverability(Recoverability::Recoverable)
    }

    /// Marks the error retryable.
    #[must_use]
    pub fn retryable(self) -> Self {
        self.with_recoverability(Recoverability::Retryable)
    }

    /// Marks the error fatal.
    #[must_use]
    pub fn fatal(self) -> Self {
        self.with_recoverability(Recoverability::Fatal)
    }

    /// Replaces structured context.
    #[must_use]
    pub fn with_context(mut self, context: ErrorContext) -> Self {
        self.inner.context = context;
        self
    }

    /// Replaces diagnostic information.
    #[must_use]
    pub fn with_diagnostic(mut self, diagnostic: Diagnostic) -> Self {
        self.inner.diagnostic = Some(diagnostic);
        self
    }

    /// Adds one machine-readable metadata entry.
    #[must_use]
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.inner.metadata.insert(key.into(), value.into());
        self
    }

    /// Adds a source error for standard error chaining.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        self.inner.source = Some(Box::new(source));
        self
    }
}

impl fmt::Display for TunexError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} error [{}]: {}",
            self.inner.kind, self.inner.code, self.inner.message
        )
    }
}

impl Error for TunexError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.inner
            .source
            .as_deref()
            .map(|source| source as &(dyn Error + 'static))
    }
}
