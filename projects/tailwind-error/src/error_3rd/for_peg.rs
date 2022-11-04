use crate::TailwindError;
use peginator::ParseError;

impl From<ParseError> for TailwindError {
    fn from(e: ParseError) -> Self {
        TailwindError::syntax_error(e.to_string())
    }
}
