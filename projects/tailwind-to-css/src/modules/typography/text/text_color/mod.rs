use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindTextColor {
    color: TailwindColor,
}

impl From<TailwindColor> for TailwindTextColor {
    fn from(color: TailwindColor) -> Self {
        Self { color }
    }
}

impl Display for TailwindTextColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "text-{}", self.color)
    }
}

impl TailwindInstance for TailwindTextColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "color" => self.color.get_properties(ctx)
        }
    }
}
