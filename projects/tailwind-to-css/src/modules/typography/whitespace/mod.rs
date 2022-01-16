use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindWhiteSpace {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindWhiteSpace => "white-space");

impl Display for TailwindWhiteSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "white-space-{}", self.kind)
    }
}

impl TailwindWhiteSpace {
    /// <https://tailwindcss.com/docs/whitespace>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("white-space", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/white-space#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "break-spaces",
            "inherit",
            "initial",
            "normal",
            "nowrap",
            "pre",
            "pre-line",
            "pre-wrap",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
