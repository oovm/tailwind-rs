use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindMargin {
    negative: Negative,
    axis: SpacingAxis,
    size: SpacingSize,
}

// noinspection DuplicatedCode
impl Display for TailwindMargin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.negative.write(f)?;
        write!(f, "{}-{}", self.axis, self.size)
    }
}

// noinspection DuplicatedCode
impl TailwindInstance for TailwindMargin {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let mut out = CssAttributes::default();
        self.axis.write_attributes(&mut out, self.size.get_properties());
        out
    }
}

// noinspection DuplicatedCode
impl TailwindMargin {
    /// https://tailwindcss.com/docs/margin
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let (axis, rest) = match pattern {
            ["m", rest @ ..] => (SpacingAxis::new("m", &["margin"]), rest),
            ["ml", rest @ ..] => (SpacingAxis::new("ml", &["margin-left"]), rest),
            ["mr", rest @ ..] => (SpacingAxis::new("mr", &["margin-right"]), rest),
            ["mt", rest @ ..] => (SpacingAxis::new("mt", &["margin-top"]), rest),
            ["mb", rest @ ..] => (SpacingAxis::new("mb", &["margin-bottom"]), rest),
            ["mx", rest @ ..] => (SpacingAxis::new("mx", &["margin-left", "margin-right"]), rest),
            ["my", rest @ ..] => (SpacingAxis::new("my", &["margin"]), rest),
            _ => return syntax_error!("Unknown margin axis"),
        };
        let size = SpacingSize::parse(rest, arbitrary, &Self::check_valid)?;
        Ok(Self { negative, axis, size })
    }
    /// https://tailwindcss.com/docs/margin#arbitrary-values
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
