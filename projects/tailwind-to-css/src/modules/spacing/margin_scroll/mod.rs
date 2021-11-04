use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindScrollMargin {
    negative: bool,
    axis: SpacingAxis,
    size: SpacingSize,
}

// noinspection DuplicatedCode
impl Display for TailwindScrollMargin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        write!(f, "{}-{}", self.axis, self.size)
    }
}

// noinspection DuplicatedCode
impl TailwindInstance for TailwindScrollMargin {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let mut out = BTreeSet::new();
        self.axis.write_attributes(&mut out, self.size.get_properties());
        out
    }
}

// noinspection DuplicatedCode
impl TailwindScrollMargin {
    /// https://tailwindcss.com/docs/scroll-margin
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: bool) -> Result<Self> {
        let axis = match pattern {
            ["m"] => SpacingAxis { class: "scroll-m", attributes: &["scroll-margin"] },
            ["mx"] => SpacingAxis { class: "scroll-mx", attributes: &["scroll-margin-left", "scroll-margin-right"] },
            ["my"] => SpacingAxis { class: "scroll-my", attributes: &["scroll-margin-top", "scroll-margin-bottom"] },
            ["ml"] => SpacingAxis { class: "scroll-ml", attributes: &["scroll-margin-left"] },
            ["mr"] => SpacingAxis { class: "scroll-mr", attributes: &["scroll-margin-right"] },
            ["mt"] => SpacingAxis { class: "scroll-mt", attributes: &["scroll-margin-top"] },
            ["mb"] => SpacingAxis { class: "scroll-mb", attributes: &["scroll-margin-bottom"] },
            _ => return syntax_error!("Unknown margin axis"),
        };
        let size = SpacingSize::parse(pattern, arbitrary)?;
        Ok(Self { negative, axis, size })
    }
    /// https://tailwindcss.com/docs/scroll-margin#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: SpacingAxis, negative: bool) -> Result<Self> {
        let size = SpacingSize::parse_arbitrary(arbitrary)?;
        Ok(Self { negative, axis, size })
    }
}
