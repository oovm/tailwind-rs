use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindIsolation {
    kind: String,
}

impl<T> From<T> for TailwindIsolation
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindIsolation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.kind.as_str();
        match self.kind.as_str() {
            "isolate" => write!(f, "isolate"),
            _ => write!(f, "isolation-{}", s),
        }
    }
}

impl TailwindInstance for TailwindIsolation {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "isolation" => self.kind
        }
    }
}

impl TailwindIsolation {
    /// https://tailwindcss.com/docs/isolation
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after isolation");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/isolation#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            // Keyword values
            "auto", "isolate", // Global values
            "inherit", "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
