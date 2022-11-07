use crate::TailwindError;
use diagnostic::ErrorWithFileSpan;
use nom::{error::Error, Err};

impl From<Err<Error<&str>>> for TailwindError {
    fn from(error: Err<Error<&str>>) -> Self {
        let e = ErrorWithFileSpan { error: error.to_string(), file: Default::default(), span: Default::default() };
        TailwindError::syntax_error(e)
    }
}
