use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindEase {
    kind: Timing,
}

#[derive(Clone, Debug)]
enum Timing {
    Standard(String),
    Arbitrary(String),
}

impl Display for Timing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standard(s) => write!(f, "{}", s),
            Self::Arbitrary(s) => write!(f, "[{}]", s),
        }
    }
}

impl Display for TailwindEase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ease-{}", self.kind)
    }
}

impl TailwindInstance for TailwindEase {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let timing = match &self.kind {
            Timing::Standard(s) => s,
            Timing::Arbitrary(s) => s,
        };
        css_attributes! {
            "transition-timing-function" => timing
        }
    }
}

impl TailwindEase {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Timing::parse(kind, arbitrary)? })
    }
}

impl Timing {
    /// https://tailwindcss.com/docs/transition-timing-function
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [s] => {
                debug_assert!(Self::check_valid(s));
                Self::Standard(s.to_string())
            },
            [] => {
                debug_assert!(arbitrary.is_some());
                Self::Arbitrary(arbitrary.to_string())
            },
            _ => return syntax_error!("Unknown ease instructions: {}", pattern.join("-")),
        };
        Ok(kind)
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/transition-timing-function#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "ease",
            "ease-in",
            "ease-in-out",
            "ease-out",
            "inherit",
            "initial",
            "linear",
            "revert",
            "step-end",
            "step-start",
            "unset",
        ]);
        set.contains(mode)
    }
}
