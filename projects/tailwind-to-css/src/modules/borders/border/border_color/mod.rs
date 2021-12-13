use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBorderColor {
    color: TailwindColor,
}

crate::macros::sealed::color_instance!(TailwindBorderColor);

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
