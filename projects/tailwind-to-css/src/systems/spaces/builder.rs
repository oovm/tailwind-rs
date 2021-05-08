use super::*;

impl TailwindInstance for TailwindPadding {
    fn id(&self) -> String {
        todo!()
    }
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {}
    }
}

impl TailwindInstance for TailwindMargin {
    fn id(&self) -> String {
        todo!()
    }
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {}
    }
}
