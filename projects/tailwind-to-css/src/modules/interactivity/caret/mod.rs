use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindCaretColor {
    color: TailwindColor,
}

impl From<TailwindColor> for TailwindCaretColor {
    fn from(color: TailwindColor) -> Self {
        Self { color }
    }
}

impl Display for TailwindCaretColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "caret-{}", self.color)
    }
}

impl TailwindInstance for TailwindCaretColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "caret-color" => self.color.get_properties(ctx),
        }
    }
}

impl TailwindCaretColor {
    /// https://tailwindcss.com/docs/caret-color
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { color: TailwindColor::parse(input, arbitrary)? })
    }
    /// https://tailwindcss.com/docs/caret-color#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { color: TailwindColor::parse_arbitrary(arbitrary)? })
    }
}
