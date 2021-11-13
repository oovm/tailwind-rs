use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontFamily {
    kind: FontFamily,
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
        Self { kind: FontFamily::Standard(kind.into()) }
    }
}

impl Display for TailwindFontFamily {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            FontFamily::Standard(s) => write!(f, "font-{}", s),
            FontFamily::Arbitrary(s) => write!(f, "font-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindFontFamily {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let family = match &self.kind {
            FontFamily::Standard(s) => ctx.fonts.get_family(s),
            FontFamily::Arbitrary(s) => s.to_owned(),
        };
        css_attributes! {
            "font-family" => family
        }
    }
}
