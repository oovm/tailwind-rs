use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindDisplay {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindDisplay => "display");

impl Display for TailwindDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "display-{}", self.kind)
    }
}

impl TailwindDisplay {
    /// <https://tailwindcss.com/docs/display>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("display", &Self::check_valid)(pattern, arbitrary)? })
    }
    /// dispatch to [display](https://developer.mozilla.org/en-US/docs/Web/CSS/display)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/display#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "block",
            "contents",
            "flex",
            "flow-root",
            "grid",
            "inherit",
            "initial",
            "inline",
            "inline-block",
            "inline-flex",
            "inline-grid",
            "list-item",
            "none",
            "revert",
            "table",
            "table-row",
        ]);
        set.contains(mode)
    }
}
