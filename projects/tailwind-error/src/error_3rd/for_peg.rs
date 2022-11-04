use diagnostic::{ErrorWithFileSpan, Span};
use peginator::ParseError;

use crate::TailwindError;

impl From<ParseError> for TailwindError {
    fn from(error: ParseError) -> Self {
        let e = ErrorWithFileSpan {
            error: error.specifics.to_string(),
            file: Default::default(),
            span: Span { start: error.position, end: error.position },
        };
        TailwindError::syntax_error(e)
    }
}
