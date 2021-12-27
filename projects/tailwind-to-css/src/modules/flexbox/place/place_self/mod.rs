use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindPlaceSelf {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindPlaceSelf => "place-self");

impl Display for TailwindPlaceSelf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "place-self-{}", self.kind)
    }
}

impl TailwindPlaceSelf {
    /// <https://tailwindcss.com/docs/place-self>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("place-self", &Self::check_valid)(pattern, arbitrary)? })
    }
    /// dispatch to [place-self](https://developer.mozilla.org/en-US/docs/Web/CSS/place-self)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-self#syntax>
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
