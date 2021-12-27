use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBoxDecoration {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindBoxDecoration => "box-decoration-break");

impl Display for TailwindBoxDecoration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                "clone" => write!(f, "box-clone"),
                "slice" => write!(f, "box-slice"),
                _ => write!(f, "box-break-{}", s),
            },
            StandardValue::Arbitrary(s) => s.write_class(f, "box-break-"),
        }
    }
}

impl TailwindBoxDecoration {
    /// <https://tailwindcss.com/docs/box-decoration-break>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("box-break", &Self::check_valid)(pattern, arbitrary)? })
    }
    /// dispatch to [box-decoration-break](https://developer.mozilla.org/en-US/docs/Web/CSS/box-decoration-break)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/box-decoration-break#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["clone", "inherit", "initial", "revert", "slice", "unset"]);
        set.contains(mode)
    }
}
