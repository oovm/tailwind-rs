use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFlexDirection {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindFlexDirection => "flex-direction");

impl Display for TailwindFlexDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                "row" => write!(f, "flex-row"),
                "row-reverse" => write!(f, "flex-row-reverse"),
                "column" => write!(f, "flex-col"),
                "column-reverse" => write!(f, "flex-col-reverse"),
                _ => write!(f, "flex-direction-{}", s),
            },
            StandardValue::Arbitrary(s) => s.write_class(f, "flex-direction-"),
        }
    }
}

impl TailwindFlexDirection {
    /// <https://tailwindcss.com/docs/flex-direction>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("flex-direction", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "column",
            "column-reverse",
            "inherit",
            "initial",
            "revert",
            "row",
            "row-reverse",
            "unset",
        ]);
        set.contains(mode)
    }
}
