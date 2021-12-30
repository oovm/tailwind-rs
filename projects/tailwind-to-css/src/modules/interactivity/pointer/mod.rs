use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindPointerEvents {
    kind: String,
}

impl<T> From<T> for TailwindPointerEvents
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindPointerEvents {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "pointer-events-{}", self.kind)
    }
}

impl TailwindInstance for TailwindPointerEvents {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "pointer-events" => self.kind
        }
    }
}

impl TailwindPointerEvents {
    /// https://tailwindcss.com/docs/pointer-events
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/pointer-events#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "all",
            "auto",
            "fill",
            "inherit",
            "initial",
            "none",
            "painted",
            "revert",
            "stroke",
            "unset",
            "visible",
            "visibleFill",
            "visiblePainted",
            "visibleStroke",
        ]);
        set.contains(mode)
    }
}
