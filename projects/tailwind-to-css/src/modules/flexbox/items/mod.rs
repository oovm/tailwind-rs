use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindItems {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindItems => "align-items");

impl Display for TailwindItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "items-{}", self.kind)
    }
}

impl TailwindItems {
    /// <https://tailwindcss.com/docs/align-items>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("items", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "baseline",
            "center",
            "end",
            "flex-end",
            "flex-start",
            "inherit",
            "initial",
            "normal",
            "revert",
            "start",
            "stretch",
            "unset",
        ]);
        set.contains(mode)
    }
}
