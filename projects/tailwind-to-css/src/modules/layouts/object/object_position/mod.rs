use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindObjectPosition {
    kind: AnchorPoint,
}

impl Display for TailwindObjectPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "object-{}", self.kind.get_class())
    }
}

impl TailwindInstance for TailwindObjectPosition {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "object-position" => self.kind.get_properties()
        }
    }
}

impl TailwindObjectPosition {
    /// <https://tailwindcss.com/docs/object-fit>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: AnchorPoint::parse(pattern, arbitrary, true)? })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit#syntax>
    pub fn check_valid(mode: &str) -> bool {
        AnchorPoint::check_valid(mode)
    }
}
