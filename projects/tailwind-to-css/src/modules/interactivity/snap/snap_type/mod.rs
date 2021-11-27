use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindSnapType {
    kind: KeywordOnly,
}

crate::macros::sealed::keyword_instance!(TailwindSnapType => "scroll-snap-type");

impl Display for TailwindSnapType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "snap-{}", self.kind)
    }
}

impl TailwindSnapType {
    /// https://tailwindcss.com/docs/scroll-snap-type
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
    /// https://tailwindcss.com/docs/scroll-snap-type
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self { kind: KeywordOnly::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/scroll-snap-type#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["block", "both", "inherit", "initial", "inline", "none", "unset", "x", "y"]);
        set.contains(mode)
    }
}
