use super::*;

#[doc = include_str!("text-decoration-style.md")]
#[derive(Debug, Clone)]
pub struct TailwindDecorationStyle {
    kind: String,
}

impl<T> From<T> for TailwindDecorationStyle
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
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
