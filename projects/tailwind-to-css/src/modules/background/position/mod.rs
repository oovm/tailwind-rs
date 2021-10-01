use super::*;

// https://tailwindcss.com/docs/background-origin
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
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "transform-origin" => self.kind.get_properties()
        }
    }
}

impl TailwindBackgroundPosition {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: AnchorPoint::parse(pattern, arbitrary)? })
    }
}
