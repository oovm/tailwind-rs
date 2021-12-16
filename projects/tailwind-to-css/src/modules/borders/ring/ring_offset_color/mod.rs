use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRingOffsetColor {
    color: TailwindColor,
}

crate::macros::sealed::color_instance!(TailwindRingOffsetColor);

impl Display for TailwindRingOffsetColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-{}", self.color)
    }
}

impl TailwindInstance for TailwindRingOffsetColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "--tw-ring-offset-color" => self.color.get_properties(ctx),
            "box-shadow" => "0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow)"
        }
    }
}
