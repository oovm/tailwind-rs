use self::axis::PaddingAxis;
use super::*;

mod axis;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindPadding {
    axis: PaddingAxis,
    size: SpaceSize,
}

impl Display for TailwindPadding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.axis, self.size)
    }
}

impl TailwindInstance for TailwindPadding {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        self.axis.get_attributes(self.size.get_properties())
    }
}

impl TailwindPadding {
    /// https://tailwindcss.com/docs/padding
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, axis: &str) -> Result<Self> {
        let axis = PaddingAxis::parse_axis(axis);
        let size = SpaceSize::parse(pattern, arbitrary)?;
        Ok(Self { axis, size })
    }
    /// https://tailwindcss.com/docs/margin
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: &str) -> Result<Self> {
        let axis = PaddingAxis::parse_axis(axis);
        let size = SpaceSize::parse_arbitrary(arbitrary)?;
        Ok(Self { axis, size })
    }
}
