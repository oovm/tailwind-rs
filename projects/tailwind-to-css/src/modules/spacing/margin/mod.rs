use self::axis::MarginAxis;
use super::*;
mod axis;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindMargin {
    negative: bool,
    axis: MarginAxis,
    size: Spacing,
}

impl Display for TailwindMargin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        write!(f, "{}-{}", self.axis, self.size)
    }
}

impl TailwindInstance for TailwindMargin {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        self.axis.get_attributes(self.size.get_properties())
    }
}

impl TailwindMargin {
    /// https://tailwindcss.com/docs/margin
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, axis: &str, negative: bool) -> Result<Self> {
        let axis = MarginAxis::parse_axis(axis);
        let size = Spacing::parse(pattern, arbitrary)?;
        Ok(Self { negative, axis, size })
    }
    /// https://tailwindcss.com/docs/margin#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: &str, negative: bool) -> Result<Self> {
        let axis = MarginAxis::parse_axis(axis);
        let size = Spacing::parse_arbitrary(arbitrary)?;
        Ok(Self { negative, axis, size })
    }
}
