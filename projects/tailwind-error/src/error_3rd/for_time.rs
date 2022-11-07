use crate::TailwindError;
use chrono::ParseError;
use diagnostic::ErrorWithFileSpan;

impl From<ParseError> for TailwindError {
    fn from(error: ParseError) -> Self {
        let e = ErrorWithFileSpan { error: error.to_string(), file: Default::default(), span: Default::default() };
        TailwindError::syntax_error(e)
    }
}
