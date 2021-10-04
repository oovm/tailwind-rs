use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBreakAfter {
    kind: String,
}

impl<T> From<T> for TailwindBreakAfter
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindBreakAfter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "break-after-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBreakAfter {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "break-after" => self.kind
        }
    }
}

impl TailwindBreakAfter {
    /// https://tailwindcss.com/docs/break-after
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after break-after");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/break-after#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            // Generic break values
            "auto",
            "avoid",
            "always",
            "all",
            // Page break values
            "avoid-page",
            "page",
            "left",
            "right",
            "recto",
            "verso",
            // Column break values
            "avoid-column",
            "column",
            // Region break values
            "avoid-region",
            "region",
            // Global values
            "inherit",
            "initial",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
