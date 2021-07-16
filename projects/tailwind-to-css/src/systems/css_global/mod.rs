use super::*;

///
#[derive(Debug, Copy, Clone)]
pub enum CssBehavior {
    Inherit,
    Initial,
    Unset,
}
impl Display for CssBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inherit => write!(f, "inherit"),
            Self::Initial => write!(f, "initial"),
            Self::Unset => write!(f, "unset"),
        }
    }
}
