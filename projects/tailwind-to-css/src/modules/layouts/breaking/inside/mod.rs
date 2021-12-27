use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBreakInside {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindBreakInside => "break-inside");

impl Display for TailwindBreakInside {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "break-inside-{}", self.kind)
    }
}

impl TailwindBreakInside {
    /// <https://tailwindcss.com/docs/break-inside>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("break-inside", &Self::check_valid)(pattern, arbitrary)? })
    }

    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-inside#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto",
            "avoid",
            "avoid-column",
            "avoid-page",
            "avoid-region",
            "inherit",
            "initial",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
