use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBottom {
    negative: bool,
    axis: Option<bool>,
    kind: PlacementSize,
}

impl Display for TailwindBottom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        match self.axis {
            None => write!(f, "bottom-{}", self.kind),
            Some(true) => write!(f, "bottom-x-{}", self.kind),
            Some(false) => write!(f, "bottom-y-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindBottom {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        todo!()
    }
}

impl TailwindBottom {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary, axis: Option<bool>, negative: bool) -> Result<Self> {
        Ok(Self { negative, axis, kind: PlacementSize::parse(kind, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: Option<bool>, negative: bool) -> Result<Self> {
        Ok(Self { negative, axis, kind: PlacementSize::parse_arbitrary(arbitrary)? })
    }
}
