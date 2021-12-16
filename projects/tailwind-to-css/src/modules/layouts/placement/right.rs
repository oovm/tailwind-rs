use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRight {
    negative: Negative,
    kind: PlacementSize,
}

impl Display for TailwindRight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.negative.write(f)?;
        write!(f, "right-{}", self.kind)
    }
}

impl TailwindInstance for TailwindRight {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "right" => self.kind.get_properties()
        }
    }
}

impl TailwindRight {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        Ok(Self { negative, kind: PlacementSize::parse(pattern, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        Ok(Self { negative, kind: PlacementSize::parse_arbitrary(arbitrary)? })
    }
}
