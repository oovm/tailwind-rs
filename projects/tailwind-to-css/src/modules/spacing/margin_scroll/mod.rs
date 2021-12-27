use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindScrollMargin {
    negative: Negative,
    axis: SpacingAxis,
    size: SpacingSize,
}

// noinspection DuplicatedCode
impl Display for TailwindScrollMargin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.negative.write(f)?;
        write!(f, "{}-{}", self.axis, self.size)
    }
}

// noinspection DuplicatedCode
impl TailwindInstance for TailwindScrollMargin {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let mut out = CssAttributes::default();
        self.axis.write_attributes(&mut out, self.size.get_properties());
        out
    }
}

// noinspection DuplicatedCode
impl TailwindScrollMargin {
    /// https://tailwindcss.com/docs/scroll-margin
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let (axis, rest) = match pattern {
            ["m", rest @ ..] => (SpacingAxis::new("scroll-m", &["scroll-margin"]), rest),
            ["ml", rest @ ..] => (SpacingAxis::new("scroll-ml", &["scroll-margin-left"]), rest),
            ["mr", rest @ ..] => (SpacingAxis::new("scroll-mr", &["scroll-margin-right"]), rest),
            ["mt", rest @ ..] => (SpacingAxis::new("scroll-mt", &["scroll-margin-top"]), rest),
            ["mb", rest @ ..] => (SpacingAxis::new("scroll-mb", &["scroll-margin-bottom"]), rest),
            ["mx", rest @ ..] => (SpacingAxis::new("scroll-mx", &["scroll-margin-left", "scroll-margin-right"]), rest),
            ["my", rest @ ..] => (SpacingAxis::new("scroll-my", &["scroll-margin"]), rest),
            _ => return syntax_error!("Unknown scroll-margin axis"),
        };
        let size = SpacingSize::parse(rest, arbitrary, &Self::check_valid)?;
        Ok(Self { negative, axis, size })
    }
    /// https://tailwindcss.com/docs/scroll-margin#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: SpacingAxis, negative: Negative) -> Result<Self> {
        let size = SpacingSize::parse_arbitrary(arbitrary)?;
        Ok(Self { negative, axis, size })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/padding#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["auto", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
