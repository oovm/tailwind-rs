use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindTableLayout {
    kind: String,
}

impl<T> From<T> for TailwindTableLayout
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindTableLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "table-{}", self.kind)
    }
}

impl TailwindInstance for TailwindTableLayout {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "table-layout" => self.kind
        }
    }
}

impl TailwindTableLayout {
    /// https://tailwindcss.com/docs/table-layout
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after table");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/table-layout#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["all", "auto", "contain", "inherit", "initial", "none", "revert", "text", "unset"]);
        set.contains(mode)
    }
}
