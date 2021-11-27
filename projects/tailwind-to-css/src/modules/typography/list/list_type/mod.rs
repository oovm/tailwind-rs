use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindListStyle {
    kind: KeywordOnly,
}

crate::macros::sealed::keyword_instance!(TailwindListStyle => "list-style-type");

impl Display for TailwindListStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            KeywordOnly::Standard(s) => match s.as_str() {
                "none" | "disc" | "decimal" => write!(f, "list-{}", s),
                _ => write!(f, "list-style-{}", s),
            },
            KeywordOnly::Arbitrary(s) => write!(f, "list-type-[{}]", s),
        }
    }
}

impl TailwindListStyle {
    /// https://tailwindcss.com/docs/list-style-type
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parser("list-style", &Self::check_valid)(pattern, arbitrary)? })
    }
    /// dispatch to [list-style-type](https://developer.mozilla.org/en-US/docs/Web/CSS/list-style-type)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parse_arbitrary(arbitrary)? })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/list-style-type#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "circle",
            "decimal",
            "disc",
            "georgian",
            "inherit",
            "initial",
            "kannada",
            "none",
            "square",
            "trad-chinese-informal",
            "unset",
        ]);
        set.contains(mode)
    }
}
