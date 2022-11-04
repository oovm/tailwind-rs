use crate::TailwindError;
use css_color::ParseColorError;
use diagnostic::ErrorWithFileSpan;

impl From<ParseColorError> for TailwindError {
    fn from(_: ParseColorError) -> Self {
        let e = ErrorWithFileSpan::new("color parse failed");
        TailwindError::syntax_error(e)
    }
}
