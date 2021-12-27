use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindWillChange {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindWillChange => "will-change");

impl Display for TailwindWillChange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "will-change-{}", self.kind)
    }
}

impl TailwindWillChange {
    /// <https://tailwindcss.com/docs/will-change>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("will-change", &Self::check_valid)(pattern, arbitrary)? })
    }
    /// dispatch to [will-change](https://developer.mozilla.org/en-US/docs/Web/CSS/will-change)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/will-change#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto",
            "contents",
            "inherit",
            "initial",
            "opacity",
            "revert",
            "scroll-position",
            "transform",
            "unset",
        ]);
        set.contains(mode)
    }
}
