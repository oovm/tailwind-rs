use super::*;

#[doc = include_str!("text-decoration-color.md")]
#[derive(Debug, Clone)]
pub struct TailwindDecorationColor {
    color: TailwindColor,
}

impl From<TailwindColor> for TailwindDecorationColor {
    fn from(color: TailwindColor) -> Self {
        Self { color }
    }
}

impl Display for TailwindDecorationColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "decoration-{}", self.color.get_class())
    }
}

impl TailwindInstance for TailwindDecorationColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "text-decoration-color" => self.color.get_properties(ctx)
        }
    }
}
