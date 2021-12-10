use super::*;

/// The [`CssBehavior`] property is specified as one of the CSS global keyword values.
///
/// *Note that none of these values affect the unicode-bidi and direction properties.*
///
/// ## Reference
/// - <https://developer.mozilla.org/en-US/docs/Web/CSS/all>
#[derive(Debug, Copy, Clone)]
pub enum CssBehavior {
    /// Specifies that all the element's properties should be changed to their initial values.
    Inherit,
    /// Specifies that all the element's properties should be changed to their inherited values.
    Initial,
    /// Specifies that all the element's properties should be changed to their inherited values if they inherit by default, or to their initial values if not.
    Unset,
    /// Specifies behavior that depends on the stylesheet origin to which the declaration belongs.
    Revert,
}

impl Display for CssBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inherit => write!(f, "inherit"),
            Self::Initial => write!(f, "initial"),
            Self::Unset => write!(f, "unset"),
            Self::Revert => write!(f, "revert"),
        }
    }
}
