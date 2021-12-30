use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindObjectFit {
    kind: String,
}

impl<T> From<T> for TailwindObjectFit
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindObjectFit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "object-{}", self.kind)
    }
}

impl TailwindInstance for TailwindObjectFit {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "object-fit" => self.kind
        }
    }
}

impl TailwindObjectFit {
    /// https://tailwindcss.com/docs/object-fit
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "contain",
            "cover",
            "fill",
            "none",
            "scale-down",
            "inherit",
            "initial",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
