use crate::TailwindError;
use nom::{error::Error, Err};

impl From<Err<Error<&str>>> for TailwindError {
    fn from(e: Err<Error<&str>>) -> Self {
        TailwindError::syntax_error(e.to_string())
    }
}
