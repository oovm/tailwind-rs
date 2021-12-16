use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindWhiteSpace {
    kind: String,
}

impl<T> From<T> for TailwindWhiteSpace
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindWhiteSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "white-space-{}", self.kind)
    }
}

impl TailwindInstance for TailwindWhiteSpace {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "white-space" => self.kind
        }
    }
}

impl TailwindWhiteSpace {
    /// https://tailwindcss.com/docs/whitespace
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after whitespace");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/white-space#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "break-spaces",
            "inherit",
            "initial",
            "normal",
            "nowrap",
            "pre",
            "pre-line",
            "pre-wrap",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
