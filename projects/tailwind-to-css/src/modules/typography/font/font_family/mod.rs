use super::*;
use crate::MaybeArbitrary;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontFamily {
    kind: MaybeArbitrary,
}

#[derive(Debug, Clone)]
enum FontFamily {
    Standard(String),
    Arbitrary(String),
}

impl<T> From<T> for TailwindFontFamily
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: MaybeArbitrary::Standard(kind.into()) }
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
            MaybeArbitrary::Standard(s) => ctx.fonts.get_family(s),
            MaybeArbitrary::Arbitrary(s) => s.to_owned(),
        };
        css_attributes! {
            "font-family" => family
        }
    }
}
