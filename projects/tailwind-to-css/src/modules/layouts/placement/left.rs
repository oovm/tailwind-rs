use super::*;

#[derive(Clone, Debug)]
pub struct TailwindLeft {
    negative: bool,
    kind: PlacementSize,
}

impl Display for TailwindLeft {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        write!(f, "left-{}", self.kind)
    }
}

impl TailwindInstance for TailwindLeft {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "left" => self.kind.get_properties()
        }
    }
}

impl TailwindLeft {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary, negative: bool) -> Result<Self> {
        Ok(Self { negative, kind: PlacementSize::parse(kind, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, negative: bool) -> Result<Self> {
        Ok(Self { negative, kind: PlacementSize::parse_unit(arbitrary)? })
    }
}
