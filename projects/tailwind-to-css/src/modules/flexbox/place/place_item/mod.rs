use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindPlaceItems {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindPlaceItems => "place-items");

impl Display for TailwindPlaceItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "place-items-{}", self.kind)
    }
}

impl TailwindPlaceItems {
    /// <https://tailwindcss.com/docs/place-self>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("place-items", &Self::check_valid)(pattern, arbitrary)? })
    }
    /// dispatch to [place-items](https://developer.mozilla.org/en-US/docs/Web/CSS/place-items)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-items#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "center",
            "end",
            "flex-end",
            "flex-start",
            "inherit",
            "initial",
            "normal",
            "revert",
            "space-around",
            "space-between",
            "space-evenly",
            "start",
            "stretch",
            "unset",
        ]);
        set.contains(mode)
    }
}
