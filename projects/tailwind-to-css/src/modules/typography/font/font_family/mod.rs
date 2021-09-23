use super::*;

// #[doc = include_str!("font-family.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontFamily {
    name: String,
}
impl TailwindInstance for TailwindFontFamily {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let family = ctx.fonts.get_family(&self.name);
        css_attributes! {
            "font-family" => family
        }
    }
}
