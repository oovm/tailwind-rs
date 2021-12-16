use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindSpace {
    axis: bool,
    negative: Negative,
    size: SpacingSize,
}

impl Display for TailwindSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.negative.write(f)?;
        match self.axis {
            true => write!(f, "space-x-{}", self.size),
            false => write!(f, "space-y-{}", self.size),
        }
    }
}

impl TailwindInstance for TailwindSpace {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = match self.axis {
            true => "margin-left",
            false => "margin-top",
        };
        css_attributes! {
            class => self.size.get_properties()
        }
    }
}

impl TailwindSpace {
    /// https://tailwindcss.com/docs/space
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Box<dyn TailwindInstance>> {
        match pattern {
            ["x", rest @ ..] => Self::parse_axis(rest, arbitrary, true, negative),
            ["y", rest @ ..] => Self::parse_axis(rest, arbitrary, false, negative),
            _ => syntax_error!("Unknown space instructions: {}", pattern.join("-")),
        }
    }
    fn parse_axis(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        axis: bool,
        negative: Negative,
    ) -> Result<Box<dyn TailwindInstance>> {
        match pattern {
            [] => Ok(Self::parse_arbitrary(arbitrary, negative, axis)?.boxed()),
            ["reverse"] => Ok(TailwindSpaceReverse::from(axis).boxed()),
            _ => {
                let size = SpacingSize::parse(pattern, arbitrary)?;
                Ok(Self { axis, negative, size }.boxed())
            },
        }
    }
    /// https://tailwindcss.com/docs/margin#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, negative: Negative, axis: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        let size = SpacingSize::parse_arbitrary(arbitrary)?;
        Ok(Self { axis, negative, size })
    }
}
