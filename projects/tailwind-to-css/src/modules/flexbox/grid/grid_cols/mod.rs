use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindGridColumns {
    kind: GridTemplate,
}

impl Display for TailwindGridColumns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "grid-cols-{}", self.kind)
    }
}

impl TailwindInstance for TailwindGridColumns {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "grid-template-columns" => self.kind.get_properties()
        }
    }
}

impl TailwindGridColumns {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: GridTemplate::parse(pattern, arbitrary)? })
    }
}
