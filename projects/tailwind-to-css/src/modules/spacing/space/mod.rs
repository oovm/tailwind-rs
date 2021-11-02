use super::*;

use self::{axis::MarginAxis, size::MarginSize};

mod axis;
mod size;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindMargin {
    axis: MarginAxis,
    size: MarginSize,
}

impl Display for TailwindMargin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, axis: &str) -> Result<Self> {
        let axis = MarginAxis::parse_axis(axis);
        let size = MarginSize::parse(pattern, arbitrary)?;
        Ok(Self { axis, size })
    }
    /// https://tailwindcss.com/docs/margin#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: &str) -> Result<Self> {
        let axis = MarginAxis::parse_axis(axis);
        let size = MarginSize::parse_arbitrary(arbitrary)?;
        Ok(Self { axis, size })
    }
}
