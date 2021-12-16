use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindCaretColor {
    color: TailwindColor,
}

crate::macros::sealed::color_instance!(TailwindCaretColor);

impl Display for TailwindCaretColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "caret-{}", self.color)
    }
}

impl TailwindInstance for TailwindCaretColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "caret-color" => self.color.get_properties(ctx),
        }
    }
}
