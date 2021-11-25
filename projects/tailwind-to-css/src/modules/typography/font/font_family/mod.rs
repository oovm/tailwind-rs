use super::*;
use crate::KeywordOnly;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontFamily {
    kind: KeywordOnly,
}

impl<T> From<T> for TailwindFontFamily
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: KeywordOnly::Standard(kind.into()) }
    }
}

impl Display for TailwindFontFamily {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "font-{}", self.kind)
    }
}

impl TailwindInstance for TailwindFontFamily {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let family = match &self.kind {
            KeywordOnly::Standard(s) => ctx.fonts.get_family(s),
            KeywordOnly::Arbitrary(s) => s.to_owned(),
        };
        css_attributes! {
            "font-family" => family
        }
    }
}
