use super::*;

impl Display for TailwindBackgroundColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-{}", self.color)
    }
}

impl TailwindInstance for TailwindBackgroundColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "background-color" => &self.color.resolve(ctx)
        }
    }
}
