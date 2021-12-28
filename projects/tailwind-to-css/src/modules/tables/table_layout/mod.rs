use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindTableLayout {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindTableLayout => "table-layout");

impl Display for TailwindTableLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "table-{}", self.kind)
    }
}

impl TailwindTableLayout {
    /// <https://tailwindcss.com/docs/table-layout>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("table-layout", &Self::check_valid)(pattern, arbitrary)? })
    }
    /// dispatch to [table-layout](https://developer.mozilla.org/en-US/docs/Web/CSS/table-layout)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parse_arbitrary(arbitrary)? })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/table-layout#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["auto", "fixed", "inherit", "initial", "unset"]);
        set.contains(mode)
    }
}
