use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBottom {
    negative: Negative,
    kind: PlacementSize,
}

impl Display for TailwindBottom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.negative.write(f)?;
        write!(f, "bottom-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBottom {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "bottom" => self.kind.get_properties()
        }
    }
}

impl TailwindBottom {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        Ok(Self { negative, kind: PlacementSize::parse(pattern, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        Ok(Self { negative, kind: PlacementSize::parse_arbitrary(arbitrary)? })
    }
}
