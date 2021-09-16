use super::*;

///
#[derive(Clone, Debug)]
pub struct TailwindRingColor {
    color: TailwindColor,
}

impl From<TailwindColor> for TailwindRingColor {
    fn from(color: TailwindColor) -> Self {
        Self { color }
    }
}

impl Display for TailwindRingColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-{}", self.color)
    }
}

impl TailwindInstance for TailwindRingColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "--tw-ring-color" => self.color.get_properties(ctx)
        }
    }
}
