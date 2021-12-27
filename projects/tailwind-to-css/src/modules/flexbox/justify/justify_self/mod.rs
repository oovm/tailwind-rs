use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindJustifySelf {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindJustifySelf => "justify-self");

impl Display for TailwindJustifySelf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "justify-self-{}", self.kind)
    }
}

impl TailwindJustifySelf {
    /// <https://tailwindcss.com/docs/justify-self>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("justify-self", &Self::check_valid)(pattern, arbitrary)? })
    }
    /// dispatch to [justify-self](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto",
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
