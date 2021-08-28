use crate::TailwindError;
use css_color::ParseColorError;

impl From<ParseColorError> for TailwindError {
    fn from(_: ParseColorError) -> Self {
        TailwindError::syntax_error("color parse failed")
    }
}
