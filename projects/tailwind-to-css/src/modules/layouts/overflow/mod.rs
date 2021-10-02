use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindOverflow {
    kind: Overflow,
    axis: Option<bool>,
}

#[derive(Clone, Debug)]
enum Overflow {
    Standard(String),
    Arbitrary(String),
}

impl Display for Overflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standard(s) => write!(f, "{}", s),
            Self::Arbitrary(s) => write!(f, "[{}]", s),
        }
    }
}

impl Display for TailwindOverflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            None => write!(f, "overflow-{}", self.kind),
            Some(true) => write!(f, "overflow-x-{}", self.kind),
            Some(false) => write!(f, "overflow-y-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindOverflow {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = match self.axis {
            None => "overflow",
            Some(true) => "overflow-x",
            Some(false) => "overflow-y",
        };
        css_attributes! {
            class => self.kind
        }
    }
}

impl TailwindOverflow {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary, axis: Option<bool>) -> Result<Self> {
        Ok(Self { kind: Overflow::parse(kind, arbitrary)?, axis })
    }
}

impl Overflow {
    /// https://tailwindcss.com/docs/overflow#header
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
            _ => return syntax_error!("Unknown overflow instructions: {}", pattern.join("-")),
        };
        Ok(kind)
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/overflow#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            // Keyword values
            "visible", "hidden", "clip", "scroll", "auto", // Global  values
            "inherit", "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
