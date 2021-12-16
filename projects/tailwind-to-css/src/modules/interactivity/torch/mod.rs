use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindTorch {
    kind: String,
}

impl<T> From<T> for TailwindTorch
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindTorch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "select-{}", self.kind)
    }
}

impl TailwindInstance for TailwindTorch {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "user-select" => self.kind
        }
    }
}

impl TailwindTorch {
    /// https://tailwindcss.com/docs/touch-action
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after torch");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/touch-action#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto",
            "inherit",
            "initial",
            "manipulation",
            "none",
            "pan-down",
            "pan-left",
            "pan-right",
            "pan-up",
            "pan-x",
            "pan-y",
            "pinch-zoom",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
