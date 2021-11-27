use super::*;
use crate::KeywordOnly;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontFamily {
    kind: KeywordOnly,
}

crate::macros::sealed::keyword_instance!(TailwindFontFamily => "font-family");

impl Display for TailwindFontFamily {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "font-{}", self.kind)
    }
}

