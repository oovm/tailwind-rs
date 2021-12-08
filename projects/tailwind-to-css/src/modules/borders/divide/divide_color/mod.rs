use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindDivideColor {
    color: TailwindColor,
}

impl From<TailwindColor> for TailwindDivideColor {
    fn from(color: TailwindColor) -> Self {
        Self { color }
    }
}

impl Display for TailwindDivideColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-{}", self.color)
    }
}

impl TailwindInstance for TailwindDivideColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "--tw-ring-color" => self.color.get_properties(ctx)
        }
    }
}
