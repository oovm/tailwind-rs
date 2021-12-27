use super::*;
use crate::StandardValue;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontFamily {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindFontFamily => "font-family");

impl Display for TailwindFontFamily {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "font-{}", self.kind)
    }
}
