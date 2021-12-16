use super::*;

#[derive(Clone, Debug)]
pub struct TailwindOrigin {
    kind: AnchorPoint,
}

impl Display for TailwindOrigin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "origin-{}", self.kind.get_class())
    }
}

impl TailwindInstance for TailwindOrigin {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "transform-origin" => self.kind.get_properties()
        }
    }
}

impl TailwindOrigin {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: AnchorPoint::parse(pattern, arbitrary)? })
    }
}
