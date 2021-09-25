use super::*;

#[derive(Debug, Clone)]
enum FontStyle {
    Standard(String),
    Arbitrary(String),
}

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontStyle {
    kind: FontStyle,
}

impl<T> From<T> for TailwindFontStyle
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: FontStyle::Standard(kind.into()) }
    }
}

impl Display for TailwindFontStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            FontStyle::Standard(s) => match s.as_str() {
                "normal" => write!(f, "not-italic"),
                "italic" => write!(f, "italic"),
                _ => write!(f, "font-style-{}", s),
            },
            FontStyle::Arbitrary(s) => write!(f, "font-style-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindFontStyle {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let style = match &self.kind {
            FontStyle::Standard(s) => s,
            FontStyle::Arbitrary(s) => s,
        };
        css_attributes! {
            "font-style" => style
        }
    }
}

impl TailwindFontStyle {
    /// https://tailwindcss.com/docs/font-style
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/font-style#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["normal", "italic", "oblique", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
