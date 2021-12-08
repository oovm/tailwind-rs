use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBorderColor {
    color: TailwindColor,
}

impl From<TailwindColor> for TailwindBorderColor {
    fn from(color: TailwindColor) -> Self {
        Self { color }
    }
}

impl Display for TailwindBorderColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "border-{}", self.color)
    }
}

impl TailwindInstance for TailwindBorderColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "border-color" => self.color.get_properties(ctx)
        }
    }
}
