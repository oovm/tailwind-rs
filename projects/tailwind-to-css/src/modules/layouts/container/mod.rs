use super::*;

#[doc=include_str!("readme.md")]
#[derive(Copy, Clone, Debug, Default)]
pub struct TailwindContainer {}

impl Display for TailwindContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "container",)
    }
}

impl TailwindInstance for TailwindContainer {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        BTreeSet::new()
    }
}
