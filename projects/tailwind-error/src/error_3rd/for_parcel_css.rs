use crate::TailwindError;
use lightningcss::error::{Error, MinifyErrorKind, ParserError, PrinterErrorKind};

impl From<Error<ParserError<'_>>> for TailwindError {
    fn from(e: Error<ParserError<'_>>) -> Self {
        TailwindError::syntax_error(e.kind.reason())
    }
}

impl From<Error<MinifyErrorKind>> for TailwindError {
    fn from(e: Error<MinifyErrorKind>) -> Self {
        TailwindError::runtime_error(e.kind.reason())
    }
}

impl From<Error<PrinterErrorKind>> for TailwindError {
    fn from(e: Error<PrinterErrorKind>) -> Self {
        TailwindError::runtime_error(e.kind.reason())
    }
}
