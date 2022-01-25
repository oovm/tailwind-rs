use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundPosition {
    kind: AnchorPoint,
}

impl Display for TailwindBackgroundPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "origin-{}", self.kind.get_class())
    }
}

impl TailwindInstance for TailwindBackgroundPosition {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "transform-origin" => self.kind.get_properties()
        }
    }
}

impl TailwindBackgroundPosition {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: AnchorPoint::parse(pattern, arbitrary, true)? })
    }
}
