use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindStrokeColor {
    color: TailwindColor,
}

impl From<TailwindColor> for TailwindStrokeColor {
    fn from(color: TailwindColor) -> Self {
        Self { color }
    }
}

impl Display for TailwindStrokeColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "stroke-{}", self.color)
    }
}

impl TailwindInstance for TailwindStrokeColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let color = self.color.get_properties(ctx);
        css_attributes! {
            "stroke" => color,
        }
    }
}

impl TailwindStrokeColor {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { color: TailwindColor::parse(input, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { color: TailwindColor::parse_arbitrary(arbitrary)? })
    }
}
