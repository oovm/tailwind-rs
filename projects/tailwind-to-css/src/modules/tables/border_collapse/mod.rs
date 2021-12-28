use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBorderCollapse {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindBorderCollapse => "border-collapse");

impl Display for TailwindBorderCollapse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                "collapse" | "separate" => write!(f, "border-{}", s),
                _ => write!(f, "border-collapse-{}", s),
            },
            StandardValue::Arbitrary(s) => s.write_class(f, "border-collapse-"),
        }
    }
}

impl TailwindBorderCollapse {
    /// <https://tailwindcss.com/docs/border-collapse>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("border-collapse", &Self::check_valid)(pattern, arbitrary)? })
    }
    /// dispatch to [border-collapse](https://developer.mozilla.org/en-US/docs/Web/CSS/border-collapse)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parse_arbitrary(arbitrary)? })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-collapse#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["collapse", "inherit", "initial", "revert", "separate", "unset"]);
        set.contains(mode)
    }
}
