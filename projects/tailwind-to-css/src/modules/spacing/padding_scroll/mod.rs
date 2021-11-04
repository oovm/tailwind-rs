use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindScrollPadding {
    negative: bool,
    axis: SpacingAxis,
    size: SpacingSize,
}

// noinspection DuplicatedCode
impl Display for TailwindScrollPadding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        write!(f, "{}-{}", self.axis, self.size)
    }
}

// noinspection DuplicatedCode
impl TailwindInstance for TailwindScrollPadding {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let mut out = BTreeSet::new();
        self.axis.write_attributes(&mut out, self.size.get_properties());
        out
    }
}

// noinspection DuplicatedCode
impl TailwindScrollPadding {
    /// https://tailwindcss.com/docs/scroll-padding
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: bool) -> Result<Self> {
        let (axis, rest) = match pattern {
            ["m", rest @ ..] => (SpacingAxis::new("scroll-p", &["scroll-padding"]), rest),
            ["ml", rest @ ..] => (SpacingAxis::new("scroll-pl", &["scroll-padding-left"]), rest),
            ["mr", rest @ ..] => (SpacingAxis::new("scroll-pr", &["scroll-padding-right"]), rest),
            ["mt", rest @ ..] => (SpacingAxis::new("scroll-pt", &["scroll-padding-top"]), rest),
            ["mb", rest @ ..] => (SpacingAxis::new("scroll-pb", &["scroll-padding-bottom"]), rest),
            ["mx", rest @ ..] => (SpacingAxis::new("scroll-px", &["scroll-padding-left", "scroll-padding-right"]), rest),
            ["my", rest @ ..] => (SpacingAxis::new("scroll-py", &["scroll-padding"]), rest),
            _ => return syntax_error!("Unknown margin axis"),
        };
        let size = SpacingSize::parse(rest, arbitrary)?;
        Ok(Self { negative, axis, size })
    }
    /// https://tailwindcss.com/docs/scroll-padding#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: SpacingAxis, negative: bool) -> Result<Self> {
        let size = SpacingSize::parse_arbitrary(arbitrary)?;
        Ok(Self { negative, axis, size })
    }
}
