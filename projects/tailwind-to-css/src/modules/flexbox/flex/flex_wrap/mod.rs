use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFlexWrap {
    kind: KeywordOnly,
}

crate::macros::sealed::keyword_instance!(TailwindFlexWrap => "flex-wrap");

impl Display for TailwindFlexWrap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            KeywordOnly::Standard(s) => match s.as_str() {
                "wrap" => write!(f, "flex-wrap"),
                "wrap-reverse" => write!(f, "flex-wrap-reverse"),
                "nowrap" => write!(f, "flex-nowrap"),
                _ => write!(f, "flex-wrap-{}", s),
            },
            KeywordOnly::Arbitrary(s) => write!(f, "flex-wrap-[{}]", s),
        }
    }
}

impl TailwindFlexWrap {
    /// https://tailwindcss.com/docs/flex-wrap
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] if arbitrary.is_none() => KeywordOnly::Standard("wrap".to_string()),
            [] => KeywordOnly::parse_arbitrary(arbitrary)?,
            ["none"] => KeywordOnly::Standard("nowrap".to_string()),
            ["reverse"] => KeywordOnly::Standard("wrap-reverse".to_string()),
            _ => KeywordOnly::parser("flex-wrap", &Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// dispatch to [flex-wrap](https://developer.mozilla.org/en-US/docs/Web/CSS/flex-wrap)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parse_arbitrary(arbitrary)? })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/flex-wrap#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "nowrap", "revert", "unset", "wrap", "wrap-reverse"]);
        set.contains(mode)
    }
}
