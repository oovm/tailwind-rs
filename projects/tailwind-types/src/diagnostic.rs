use std::fmt;

use miette::{Diagnostic as MietteDiagnostic, LabeledSpan, SourceSpan};
use serde::{Deserialize, Serialize};

/// Structured engine diagnostic. Never panic on user input.
///
/// Implements [`std::error::Error`] and [`miette::Diagnostic`] so hosts can
/// render rich reports without inventing a parallel error type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: Severity,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub candidate_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<TextRange>,
    /// Optional source text for miette snippets (not part of Canonical Module).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Stable diagnostic codes. Extend carefully; codes are conformance surface.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCode {
    /// Resolve / normalize not wired yet (parser may still succeed).
    EngineNotReady,
    ParseIncomplete,
    ParseResidual,
    ParseInvalidEscape,
    ParseUnbalanced,
    ParseEmpty,
    ResolveUnknownUtility,
    ResolveAmbiguous,
    ResolveNegativeNotAllowed,
    ResolveModifierNotAllowed,
    ResolveInvalidType,
    ThemeMissing,
    ThemeAmbiguous,
    ThemeInvalidType,
    ThemeAliasCycle,
    VariantUnknown,
    VariantNotCombinable,
    CanonicalConflict,
    /// `CompileRequest.contributions` is non-empty but the contribution protocol is not executed yet.
    ContributionUnsupported,
    Other(String),
}

impl DiagnosticCode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::EngineNotReady => "tailwind::engine_not_ready",
            Self::ParseIncomplete => "tailwind::parse::incomplete",
            Self::ParseResidual => "tailwind::parse::residual",
            Self::ParseInvalidEscape => "tailwind::parse::invalid_escape",
            Self::ParseUnbalanced => "tailwind::parse::unbalanced",
            Self::ParseEmpty => "tailwind::parse::empty",
            Self::ResolveUnknownUtility => "tailwind::resolve::unknown_utility",
            Self::ResolveAmbiguous => "tailwind::resolve::ambiguous",
            Self::ResolveNegativeNotAllowed => "tailwind::resolve::negative_not_allowed",
            Self::ResolveModifierNotAllowed => "tailwind::resolve::modifier_not_allowed",
            Self::ResolveInvalidType => "tailwind::resolve::invalid_type",
            Self::ThemeMissing => "tailwind::theme::missing",
            Self::ThemeAmbiguous => "tailwind::theme::ambiguous",
            Self::ThemeInvalidType => "tailwind::theme::invalid_type",
            Self::ThemeAliasCycle => "tailwind::theme::alias_cycle",
            Self::VariantUnknown => "tailwind::variant::unknown",
            Self::VariantNotCombinable => "tailwind::variant::not_combinable",
            Self::CanonicalConflict => "tailwind::canonical::conflict",
            Self::ContributionUnsupported => "tailwind::contribution::unsupported",
            Self::Other(s) => s.as_str(),
        }
    }
}

impl Diagnostic {
    pub fn error(code: DiagnosticCode, message: impl Into<String>) -> Self {
        Self {
            code,
            severity: Severity::Error,
            message: message.into(),
            candidate_key: None,
            span: None,
            source: None,
        }
    }

    pub fn with_candidate_key(mut self, key: impl Into<String>) -> Self {
        self.candidate_key = Some(key.into());
        self
    }

    pub fn with_span(mut self, span: TextRange) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Diagnostic {}

impl MietteDiagnostic for Diagnostic {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        Some(Box::new(self.code.as_str()))
    }

    fn severity(&self) -> Option<miette::Severity> {
        Some(match self.severity {
            Severity::Error => miette::Severity::Error,
            Severity::Warning => miette::Severity::Warning,
            Severity::Info => miette::Severity::Advice,
        })
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        let span = self.span?;
        let label = LabeledSpan::new(
            Some(self.message.clone()),
            span.start as usize,
            (span.end.saturating_sub(span.start)) as usize,
        );
        Some(Box::new(std::iter::once(label)))
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        self.source.as_ref().map(|s| s as &dyn miette::SourceCode)
    }
}

/// Byte range within a single candidate token text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TextRange {
    pub start: u32,
    pub end: u32,
}

impl TextRange {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn as_source_span(self) -> SourceSpan {
        SourceSpan::new(
            (self.start as usize).into(),
            self.end.saturating_sub(self.start) as usize,
        )
    }
}
