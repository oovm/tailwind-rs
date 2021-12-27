use super::*;
use crate::StandardValue;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindEase {
    kind: StandardValue,
}
crate::macros::sealed::keyword_instance!(TailwindEase => "transition-timing-function");

impl Display for TailwindEase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ease-{}", self.kind)
    }
}

impl TailwindEase {
    /// https://tailwindcss.com/docs/transition-timing-function
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("ease", &Self::check_valid)(pattern, arbitrary)? })
    }
    /// https://tailwindcss.com/docs/transition-timing-function#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/transition-timing-function#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "ease",
            "ease-in",
            "ease-in-out",
            "ease-out",
            "inherit",
            "initial",
            "linear",
            "revert",
            "step-end",
            "step-start",
            "unset",
        ]);
        set.contains(mode)
    }
}
