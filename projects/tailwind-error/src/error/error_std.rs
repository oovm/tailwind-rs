use std::num::{ParseFloatError, ParseIntError};

use super::*;

impl From<std::io::Error> for TailwindError {
    fn from(error: std::io::Error) -> Self {
        let e = ErrorWithFile::new(error.to_string());
        Self { kind: Box::new(TailwindErrorKind::IOError(e)), level: DiagnosticLevel::Error }
    }
}
impl From<std::fmt::Error> for TailwindError {
    fn from(e: std::fmt::Error) -> Self {
        Self { kind: Box::new(TailwindErrorKind::FormatError(e)), level: DiagnosticLevel::Error }
    }
}

impl From<Infallible> for TailwindError {
    fn from(_: Infallible) -> Self {
        Self::unreachable()
    }
}

impl From<()> for TailwindError {
    fn from(_: ()) -> Self {
        Self::unreachable()
    }
}

impl From<ParseIntError> for TailwindError {
    fn from(e: ParseIntError) -> Self {
        Self::syntax_error(e.to_string())
    }
}

impl From<ParseFloatError> for TailwindError {
    fn from(e: ParseFloatError) -> Self {
        Self::syntax_error(e.to_string())
    }
}
