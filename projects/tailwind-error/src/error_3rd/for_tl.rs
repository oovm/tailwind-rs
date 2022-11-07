use diagnostic::ErrorWithFileSpan;
use tl::ParseError;

use crate::TailwindError;

impl From<ParseError> for TailwindError {
    fn from(error: ParseError) -> Self {
        let e = ErrorWithFileSpan { error: error.to_string(), file: Default::default(), span: Default::default() };
        TailwindError::syntax_error(e)
    }
}
