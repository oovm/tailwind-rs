use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindPadding {
    negative: bool,
    axis: SpacingAxis,
    size: SpacingSize,
}

// noinspection DuplicatedCode
impl Display for TailwindPadding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        write!(f, "{}-{}", self.axis, self.size)
    }
}

// noinspection DuplicatedCode
impl TailwindInstance for TailwindPadding {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let mut out = BTreeSet::new();
        self.axis.write_attributes(&mut out, self.size.get_properties());
        out
    }
}

// noinspection DuplicatedCode
impl TailwindPadding {
    /// https://tailwindcss.com/docs/padding
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: bool) -> Result<Self> {
        let (axis, rest) = match pattern {
            ["p", rest @ ..] => (SpacingAxis::new("p", &["padding"]), rest),
            ["pl", rest @ ..] => (SpacingAxis::new("pl", &["padding-left"]), rest),
            ["pr", rest @ ..] => (SpacingAxis::new("pr", &["padding-right"]), rest),
            ["pt", rest @ ..] => (SpacingAxis::new("pt", &["padding-top"]), rest),
            ["pb", rest @ ..] => (SpacingAxis::new("pb", &["padding-bottom"]), rest),
            ["px", rest @ ..] => (SpacingAxis::new("px", &["padding-left", "padding-right"]), rest),
            ["py", rest @ ..] => (SpacingAxis::new("py", &["padding"]), rest),
            _ => return syntax_error!("Unknown padding axis"),
        };
        let size = SpacingSize::parse(rest, arbitrary)?;
        Ok(Self { negative, axis, size })
    }
    /// https://tailwindcss.com/docs/padding#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: SpacingAxis, negative: bool) -> Result<Self> {
        let size = SpacingSize::parse_arbitrary(arbitrary)?;
        Ok(Self { negative, axis, size })
    }
}
