use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindJustifyItems {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindJustifyItems => "justify-items");

impl Display for TailwindJustifyItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "justify-items-{}", self.kind)
    }
}

impl TailwindJustifyItems {
    /// <https://tailwindcss.com/docs/justify-items>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("justify-items", &Self::check_valid)(pattern, arbitrary)? })
    }
    /// dispatch to [justify-items](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "baseline",
            "center",
            "end",
            "flex-end",
            "flex-start",
            "inherit",
            "initial",
            "left",
            "normal",
            "revert",
            "right",
            "self-end",
            "self-start",
            "start",
            "stretch",
            "unset",
        ]);
        set.contains(mode)
    }
}
