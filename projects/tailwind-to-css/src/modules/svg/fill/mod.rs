use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFillColor {
    color: TailwindColor,
}

crate::macros::sealed::color_instance!(TailwindFillColor);

impl Display for TailwindFillColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "fill-{}", self.color)
    }
}

impl TailwindInstance for TailwindFillColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let color = self.color.get_properties(ctx);
        css_attributes! {
            "fill" => color,
        }
    }
}
