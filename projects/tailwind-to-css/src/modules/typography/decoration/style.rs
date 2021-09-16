use super::*;

#[doc = include_str!("text-decoration-style.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindDecorationStyle {
    kind: &'static str,
}

impl From<&'static str> for TailwindDecorationStyle {
    fn from(kind: &'static str) -> Self {
        Self { kind }
    }
}

impl Display for TailwindDecorationStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "decoration-{}", self.kind)
    }
}

impl TailwindInstance for TailwindDecorationStyle {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "text-decoration-style" => self.kind
        }
    }
}
