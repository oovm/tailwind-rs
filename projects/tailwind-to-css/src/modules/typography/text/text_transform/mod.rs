use super::*;
use std::fmt::Write;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindTextTransform {
    kind: String,
}

impl<T> From<T> for TailwindTextTransform
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindTextTransform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "text-transform-{}", self.kind)
    }
}

impl TailwindInstance for TailwindTextTransform {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "text-transform" => self.kind
        }
    }
}

impl TailwindTextTransform {
    /// https://tailwindcss.com/docs/text-transform
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after text-transform");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/text-transform#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "none",
            "capitalize",
            "uppercase",
            "lowercase",
            "full-width",
            "full-size-kana",
            "inherit",
            "initial",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
