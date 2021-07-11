use super::*;

impl Display for TailwindShadow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindShadow {}

impl Display for TailwindOpacity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let normed = self.opacity.min(100);
        write!(f, "opacity-{}", normed)
    }
}

impl TailwindInstance for TailwindOpacity {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let opacity = format!("{}", self.opacity.min(100) as f32 / 100.0);
        css_attributes! {
            "opacity" => opacity
        }
    }
}

impl Display for TailwindBlend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindBlend {
    fn selectors(&self, ctx: &TailwindBuilder) -> String {
        todo!()
    }
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        todo!()
    }
}
