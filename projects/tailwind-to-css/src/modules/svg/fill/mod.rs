use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFillColor {
    color: TailwindColor,
}

impl From<TailwindColor> for TailwindFillColor {
    fn from(color: TailwindColor) -> Self {
        Self { color }
    }
}

impl Display for TailwindFillColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "fill-{}", self.color)
    }
}

impl TailwindInstance for TailwindFillColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let color = self.color.get_properties(ctx);
        css_attributes! {
            "fill" => color,
        }
    }
}

impl TailwindFillColor {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { color: TailwindColor::parse(input, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { color: TailwindColor::parse_arbitrary(arbitrary)? })
    }
}
