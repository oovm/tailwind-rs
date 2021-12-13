use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindStrokeColor {
    color: TailwindColor,
}

crate::macros::sealed::color_instance!(TailwindStrokeColor);

impl Display for TailwindStrokeColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "stroke-{}", self.color)
    }
}

impl TailwindInstance for TailwindStrokeColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let color = self.color.get_properties(ctx);
        css_attributes! {
            "stroke" => color,
        }
    }
}
