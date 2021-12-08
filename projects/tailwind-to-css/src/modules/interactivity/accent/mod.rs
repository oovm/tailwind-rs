use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindAccentColor {
    color: TailwindColor,
}

impl From<TailwindColor> for TailwindAccentColor {
    fn from(color: TailwindColor) -> Self {
        Self { color }
    }
}

impl Display for TailwindAccentColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "accent-{}", self.color)
    }
}

impl TailwindInstance for TailwindAccentColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "accent-color" => self.color.get_properties(ctx),
        }
    }
}

impl TailwindAccentColor {
    /// https://tailwindcss.com/docs/accent-color
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { color: TailwindColor::parse(input, arbitrary)? })
    }
    /// https://tailwindcss.com/docs/accent-color#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { color: TailwindColor::parse_arbitrary(arbitrary)? })
    }
}
