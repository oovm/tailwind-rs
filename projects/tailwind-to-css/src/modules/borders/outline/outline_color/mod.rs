use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindOutlineColor {
    color: TailwindColor,
}

crate::macros::sealed::color_instance!(TailwindOutlineColor);

impl Display for TailwindOutlineColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "outline-color-{}", self.color)
    }
}

impl TailwindInstance for TailwindOutlineColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "outline-color" => self.color.get_properties(ctx)
        }
    }
}
