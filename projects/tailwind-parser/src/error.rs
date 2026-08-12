use miette::{Diagnostic, SourceSpan};
use thiserror::Error;
use tailwind_types::{Diagnostic as EngineDiagnostic, DiagnosticCode, TextRange};

pub type ParseResult<T> = Result<T, ParseError>;

/// Parser / lexer failure with miette metadata.
#[derive(Debug, Error, Diagnostic, Clone, PartialEq, Eq)]
pub enum ParseError {
    #[error("empty input")]
    #[diagnostic(code(tailwind::parse::empty))]
    Empty,

    #[error("incomplete candidate")]
    #[diagnostic(code(tailwind::parse::incomplete))]
    Incomplete {
        #[label("expected more input here")]
        span: SourceSpan,
    },

    #[error("unexpected characters after candidate: {residual:?}")]
    #[diagnostic(code(tailwind::parse::residual))]
    Residual {
        residual: String,
        #[label("residual starts here")]
        span: SourceSpan,
    },

    #[error("invalid escape sequence")]
    #[diagnostic(code(tailwind::parse::invalid_escape))]
    InvalidEscape {
        #[label("bad escape")]
        span: SourceSpan,
    },

    #[error("unbalanced `{opener}` / `{closer}`")]
    #[diagnostic(code(tailwind::parse::unbalanced))]
    Unbalanced {
        opener: char,
        closer: char,
        #[label("opens here")]
        span: SourceSpan,
    },

    #[error("{message}")]
    #[diagnostic(code(tailwind::parse::incomplete))]
    Unexpected {
        message: String,
        #[label("here")]
        span: SourceSpan,
    },
}

impl ParseError {
    pub fn into_diagnostic(self, source: &str) -> EngineDiagnostic {
        let (code, message, span) = match &self {
            Self::Empty => (DiagnosticCode::ParseEmpty, self.to_string(), None),
            Self::Incomplete { span } => (
                DiagnosticCode::ParseIncomplete,
                self.to_string(),
                Some(span_to_range(*span)),
            ),
            Self::Residual { span, .. } => (
                DiagnosticCode::ParseResidual,
                self.to_string(),
                Some(span_to_range(*span)),
            ),
            Self::InvalidEscape { span } => (
                DiagnosticCode::ParseInvalidEscape,
                self.to_string(),
                Some(span_to_range(*span)),
            ),
            Self::Unbalanced { span, .. } => (
                DiagnosticCode::ParseUnbalanced,
                self.to_string(),
                Some(span_to_range(*span)),
            ),
            Self::Unexpected { span, .. } => (
                DiagnosticCode::ParseIncomplete,
                self.to_string(),
                Some(span_to_range(*span)),
            ),
        };

        let mut d = EngineDiagnostic::error(code, message).with_source(source.to_string());
        if let Some(span) = span {
            d = d.with_span(span);
        }
        d
    }

    pub fn into_diagnostics(self, source: &str) -> Vec<EngineDiagnostic> {
        vec![self.into_diagnostic(source)]
    }
}

fn span_to_range(span: SourceSpan) -> TextRange {
    let start = span.offset() as u32;
    let end = start + span.len() as u32;
    TextRange::new(start, end)
}
