use crate::JssError;
use serde_json::Error;

impl From<Error> for JssError {
    fn from(e: Error) -> Self {
        JssError::syntax_error(e.to_string())
    }
}
