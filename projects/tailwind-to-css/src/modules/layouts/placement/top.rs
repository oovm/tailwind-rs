use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindTop {
    negative: bool,
    kind: PlacementSize,
}

impl Display for TailwindTop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        write!(f, "top-{}", self.kind)
    }
}

impl TailwindInstance for TailwindTop {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "top" => self.kind.get_properties()
        }
    }
}

impl TailwindTop {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary, negative: bool) -> Result<Self> {
        Ok(Self { negative, kind: PlacementSize::parse(kind, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, negative: bool) -> Result<Self> {
        Ok(Self { negative, kind: PlacementSize::parse_arbitrary(arbitrary)? })
    }
}
