use crate::KeywordOnly;

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindOverscroll {
    kind: KeywordOnly,
    axis: Option<bool>,
}

impl Display for TailwindOverscroll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            None => write!(f, "overscroll-{}", self.kind),
            Some(true) => write!(f, "overscroll-x-{}", self.kind),
            Some(false) => write!(f, "overscroll-y-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindOverscroll {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = match self.axis {
            None => "overscroll-behavior",
            Some(true) => "overscroll-behavior-x",
            Some(false) => "overscroll-behavior-y",
        };
        css_attributes! {
            class => self.kind
        }
    }
}

impl TailwindOverscroll {
    /// https://tailwindcss.com/docs/overscroll-behavior
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary, axis: Option<bool>) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parser("overscroll", &Self::check_valid)(kind, arbitrary)?, axis })
    }
    /// https://tailwindcss.com/docs/overscroll-behavior#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: Option<bool>) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parse_arbitrary(arbitrary)?, axis })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/overscroll-behavior#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["auto", "contain", "inherit", "initial", "none", "revert", "unset"]);
        set.contains(mode)
    }
}
