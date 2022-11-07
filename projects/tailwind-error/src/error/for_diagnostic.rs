use diagnostic::{DiagnosticError, ErrorWithFileSpan};

use crate::TailwindError;

impl From<DiagnosticError> for TailwindError {
    fn from(error: DiagnosticError) -> Self {
        let e = ErrorWithFileSpan::new(error.to_string());
        TailwindError::syntax_error(e)
    }
}
