use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindListPosition {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindListPosition => "list-style-position");

impl Display for TailwindListPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                "inside" | "outside" => write!(f, "list-{}", s),
                _ => write!(f, "list-position-{}", s),
            },
            StandardValue::Arbitrary(s) => s.write_class(f, "list-position-"),
        }
    }
}

impl TailwindListPosition {
    /// <https://tailwindcss.com/docs/list-style-position>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("list-position", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/list-style-position#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "inside", "outside", "unset"]);
        set.contains(mode)
    }
}
