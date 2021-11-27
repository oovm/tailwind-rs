use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindSnapAlign {
    kind: KeywordOnly,
}

crate::macros::sealed::keyword_instance!(TailwindSnapAlign => "scroll-snap-align");

impl Display for TailwindSnapAlign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            KeywordOnly::Standard(s) => match s.as_str() {
                "start" => write!(f, "snap-start"),
                "end" => write!(f, "snap-end"),
                "center" => write!(f, "snap-center"),
                _ => write!(f, "snap-align-{}", s),
            },
            KeywordOnly::Arbitrary(s) => write!(f, "snap-align-[{}]", s),
        }
    }
}

impl TailwindSnapAlign {
    /// https://tailwindcss.com/docs/cursor
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: KeywordOnly::Standard(s) })
            },
        }
    }
    /// https://tailwindcss.com/docs/scroll-snap-align
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self { kind: KeywordOnly::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/scroll-snap-align#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["center", "inherit", "initial", "none", "revert", "unset"]);
        set.contains(mode)
    }
}
