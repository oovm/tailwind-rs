use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindAccentColor {
    color: TailwindColor,
}

crate::macros::sealed::color_instance!(TailwindAccentColor);

impl Display for TailwindAccentColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "accent-{}", self.color)
    }
}

impl TailwindInstance for TailwindAccentColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "accent-color" => self.color.get_properties(ctx),
        }
    }
}
