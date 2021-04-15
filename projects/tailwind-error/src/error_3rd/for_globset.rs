use crate::JssError;
use globset::Error;

impl From<Error> for JssError {
    fn from(e: Error) -> Self {
        JssError::runtime_error(e.to_string())
    }
}
