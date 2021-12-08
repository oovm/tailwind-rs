use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBottom {
    negative: bool,
    kind: PlacementSize,
}

impl Display for TailwindBottom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        write!(f, "bottom-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBottom {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "bottom" => self.kind.get_properties()
        }
    }
}

impl TailwindBottom {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary, negative: bool) -> Result<Self> {
        Ok(Self { negative, kind: PlacementSize::parse(kind, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, negative: bool) -> Result<Self> {
        Ok(Self { negative, kind: PlacementSize::parse_arbitrary(arbitrary)? })
    }
}
