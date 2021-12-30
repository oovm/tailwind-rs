use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindListPosition {
    kind: String,
}

impl<T> From<T> for TailwindListPosition
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindListPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.kind.as_str();
        match s {
            "inside" | "outside" => write!(f, "list-{}", s),
            _ => write!(f, "list-position-{}", s),
        }
    }
}

impl TailwindInstance for TailwindListPosition {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "resize" => self.kind
        }
    }
}

impl TailwindListPosition {
    /// https://tailwindcss.com/docs/list-style-position
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/list-style-position#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "inside", "outside", "unset"]);
        set.contains(mode)
    }
}
