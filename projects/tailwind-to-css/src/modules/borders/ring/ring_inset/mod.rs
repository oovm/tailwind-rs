use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone, Default)]
pub struct TailwindRingInset {}

impl Display for TailwindRingInset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-inset")
    }
}

impl TailwindInstance for TailwindRingInset {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "--tw-ring-inset" => "inset"
        }
    }
}
