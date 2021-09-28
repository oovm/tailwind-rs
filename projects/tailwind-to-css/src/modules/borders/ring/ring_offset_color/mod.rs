use super::*;

///
#[derive(Clone, Debug)]
pub struct TailwindRingOffsetColor {
    color: TailwindColor,
}

impl From<TailwindColor> for TailwindRingOffsetColor {
    fn from(color: TailwindColor) -> Self {
        Self { color }
    }
}

impl Display for TailwindRingOffsetColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-{}", self.color)
    }
}

impl TailwindInstance for TailwindRingOffsetColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "--tw-ring-offset-color" => self.color.get_properties(ctx),
            "box-shadow" => "0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow)"
        }
    }
}