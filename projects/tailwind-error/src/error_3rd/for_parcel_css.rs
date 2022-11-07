use crate::TailwindError;
use diagnostic::ErrorWithFileSpan;
use parcel_css::error::{Error, MinifyErrorKind, ParserError, PrinterErrorKind};

impl From<Error<ParserError<'_>>> for TailwindError {
    fn from(error: Error<ParserError<'_>>) -> Self {
        // match error.loc {
        //     None => {},
        //     Some(s) => s.filename,
        // }
        let e = ErrorWithFileSpan { error: error.kind.to_string(), file: Default::default(), span: Default::default() };
        TailwindError::syntax_error(e)
    }
}

impl From<Error<MinifyErrorKind>> for TailwindError {
    fn from(e: Error<MinifyErrorKind>) -> Self {
        TailwindError::runtime_error(e.kind.to_string())
    }
}

impl From<Error<PrinterErrorKind>> for TailwindError {
    fn from(e: Error<PrinterErrorKind>) -> Self {
        TailwindError::runtime_error(e.kind.to_string())
    }
}
