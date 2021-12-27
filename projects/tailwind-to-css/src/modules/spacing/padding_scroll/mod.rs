use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindScrollPadding {
    negative: Negative,
    axis: SpacingAxis,
    size: SpacingSize,
}

// noinspection DuplicatedCode
impl Display for TailwindScrollPadding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.negative.write(f)?;
        write!(f, "{}-{}", self.axis, self.size)
    }
}

// noinspection DuplicatedCode
impl TailwindInstance for TailwindScrollPadding {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let mut out = CssAttributes::default();
        self.axis.write_attributes(&mut out, self.size.get_properties());
        out
    }
}

// noinspection DuplicatedCode
impl TailwindScrollPadding {
    /// https://tailwindcss.com/docs/scroll-padding
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let (axis, rest) = match pattern {
            ["p", rest @ ..] => (SpacingAxis::new("scroll-p", &["scroll-padding"]), rest),
            ["pl", rest @ ..] => (SpacingAxis::new("scroll-pl", &["scroll-padding-left"]), rest),
            ["pr", rest @ ..] => (SpacingAxis::new("scroll-pr", &["scroll-padding-right"]), rest),
            ["pt", rest @ ..] => (SpacingAxis::new("scroll-pt", &["scroll-padding-top"]), rest),
            ["pb", rest @ ..] => (SpacingAxis::new("scroll-pb", &["scroll-padding-bottom"]), rest),
            ["px", rest @ ..] => (SpacingAxis::new("scroll-px", &["scroll-padding-left", "scroll-padding-right"]), rest),
            ["py", rest @ ..] => (SpacingAxis::new("scroll-py", &["scroll-padding"]), rest),
            _ => return syntax_error!("Unknown scroll-padding axis"),
        };
        let size = SpacingSize::parse(rest, arbitrary, &Self::check_valid)?;
        Ok(Self { negative, axis, size })
    }
    /// https://tailwindcss.com/docs/scroll-padding#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: SpacingAxis, negative: Negative) -> Result<Self> {
        let size = SpacingSize::parse_arbitrary(arbitrary)?;
        Ok(Self { negative, axis, size })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/padding#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
