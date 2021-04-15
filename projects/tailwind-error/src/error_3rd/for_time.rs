use crate::JssError;
use chrono::ParseError;

impl From<ParseError> for JssError {
    fn from(e: ParseError) -> Self {
        JssError::syntax_error(e.to_string())
    }
}
