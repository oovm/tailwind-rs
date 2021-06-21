use super::*;

impl Display for TailwindShadow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindShadow {}

impl Display for TailwindOpacity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindOpacity {
    fn selectors(&self, ctx: &TailwindBuilder) -> String {
        todo!()
    }
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        todo!()
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
