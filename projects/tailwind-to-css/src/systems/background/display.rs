use super::*;

impl TailwindInstance for TailwindBackgroundColor {
    fn id(&self) -> String {
        format!("bg- {};", self.color)
    }
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "aspect-ratio" => self.ratio
        }
    }
}
