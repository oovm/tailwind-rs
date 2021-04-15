use crate::JssError;
use git2::Error;

impl From<Error> for JssError {
    fn from(e: Error) -> Self {
        JssError::runtime_error(e.to_string())
    }
}
