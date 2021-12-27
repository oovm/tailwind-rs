use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindBoxSizing {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindBoxSizing => "box-sizing");

impl Display for TailwindBoxSizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                "border-box" => write!(f, "box-border"),
                "content-box" => write!(f, "box-content"),
                _ => write!(f, "box-sizing-{}", s),
            },
            StandardValue::Arbitrary(s) => s.write_class(f, "box-sizing-"),
        }
    }
}

impl TailwindBoxSizing {
    /// <https://tailwindcss.com/docs/box-sizing>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("box-sizing", &Self::check_valid)(pattern, arbitrary)? })
    }
    /// dispatch to [box-sizing](https://developer.mozilla.org/en-US/docs/Web/CSS/box-sizing)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["border-box", "content-box", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
