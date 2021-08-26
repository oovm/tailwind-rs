use crate::{TailwindError, TailwindErrorKind};
use css_color::ParseColorError;
use yggdrasil_shared::DiagnosticLevel;

impl From<ParseColorError> for TailwindError {
    fn from(_: ParseColorError) -> Self {
        TailwindError::syntax_error("color parse failed")
    }
}
