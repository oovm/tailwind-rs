use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBreakAfter {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindBreakAfter => "break-after");

impl Display for TailwindBreakAfter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "break-after-{}", self.kind)
    }
}

impl TailwindBreakAfter {
    /// <https://tailwindcss.com/docs/break-after>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("break-after", &Self::check_valid)(pattern, arbitrary)? })
    }

    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-after#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "all",
            "always",
            "auto",
            "avoid",
            "avoid-column",
            "avoid-page",
            "avoid-region",
            "column",
            "inherit",
            "initial",
            "left",
            "page",
            "recto",
            "region",
            "revert",
            "right",
            "unset",
            "verso",
        ]);
        set.contains(mode)
    }
}
