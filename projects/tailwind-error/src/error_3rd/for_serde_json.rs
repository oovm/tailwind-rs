use crate::TailwindError;
use serde_json::Error;

impl From<Error> for TailwindError {
    fn from(e: Error) -> Self {
        TailwindError::syntax_error(e.to_string())
    }
}
