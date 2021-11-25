use super::*;
use crate::KeywordOnly;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindEase {
    kind: KeywordOnly,
}

impl Display for TailwindEase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ease-{}", self.kind)
    }
}

impl TailwindInstance for TailwindEase {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let timing = match &self.kind {
            KeywordOnly::Standard(s) => s,
            KeywordOnly::Arbitrary(s) => s,
        };
        css_attributes! {
            "transition-timing-function" => timing
        }
    }
}

impl TailwindEase {
    /// https://tailwindcss.com/docs/transition-timing-function
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parser("ease", &check_valid)(kind, arbitrary)? })
    }
    /// https://tailwindcss.com/docs/transition-timing-function#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parse_arbitrary(arbitrary)? })
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/transition-timing-function#syntax
fn check_valid(mode: &str) -> bool {
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
