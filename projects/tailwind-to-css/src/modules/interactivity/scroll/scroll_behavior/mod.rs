use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindScrollBehavior {
    kind: String,
}

impl<T> From<T> for TailwindScrollBehavior
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindScrollBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "scroll-behavior-{}", self.kind)
    }
}

impl TailwindInstance for TailwindScrollBehavior {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "scroll-behavior" => self.kind
        }
    }
}

impl TailwindScrollBehavior {
    /// https://tailwindcss.com/docs/scroll-behavior
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/scroll-behavior#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["auto", "inherit", "initial", "revert", "smooth", "unset"]);
        set.contains(mode)
    }
}
