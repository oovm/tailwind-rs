use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindDecorationLine {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindDecorationLine => "text-decoration-line");

impl Display for TailwindDecorationLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                "underline" | "overline" | "line-through" => write!(f, "{}", s),
                "none" => write!(f, "no-underline"),
                _ => write!(f, "decoration-line-{}", s),
            },
            StandardValue::Arbitrary(s) => write!(f, "decoration-line-{}", s.get_class()),
        }
    }
}

impl TailwindDecorationLine {
    /// https://tailwindcss.com/docs/cursor
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: StandardValue::Keyword(s) })
            },
        }
    }
    /// https://tailwindcss.com/docs/cursor#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::Arbitrary(arbitrary.to_owned()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set =
            BTreeSet::from_iter(vec!["blink", "inherit", "initial", "line-through", "none", "overline", "underline", "unset"]);
        set.contains(mode)
    }
}
