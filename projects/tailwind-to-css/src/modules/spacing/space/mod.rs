use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindSpace {
    axis: bool,
    reverse: bool,
    size: SpaceSize,
}

impl Display for TailwindSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.axis, self.size)
    }
}

impl TailwindInstance for TailwindSpace {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        self.axis.get_attributes(self.size.get_properties())
    }
}

impl TailwindSpace {
    /// https://tailwindcss.com/docs/margin
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, axis: &str) -> Result<Self> {
        let axis = SpaceAxis::parse_axis(axis);
        let size = SpaceSize::parse(pattern, arbitrary)?;
        Ok(Self { axis, size })
    }
    /// https://tailwindcss.com/docs/margin#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: &str) -> Result<Self> {
        let axis = SpaceAxis::parse_axis(axis);
        let size = SpaceSize::parse_arbitrary(arbitrary)?;
        Ok(Self { axis, size })
    }
}
