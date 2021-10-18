use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindDecorationStyle {
    kind: String,
}

impl<T> From<T> for TailwindDecorationStyle
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindDecorationStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.kind.as_str();
        match s {
            "solid" | "double" | "dotted" | "dashed" | "wavy" => write!(f, "decoration-{}", s),
            _ => write!(f, "decoration-style-{}", s),
        }
    }
}

impl TailwindInstance for TailwindDecorationStyle {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "text-decoration-style" => self.kind
        }
    }
}

impl TailwindDecorationStyle {
    /// https://tailwindcss.com/docs/object-fit
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after decoration-style");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-style#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set =
            BTreeSet::from_iter(vec!["dashed", "dotted", "double", "inherit", "initial", "revert", "solid", "unset", "wavy"]);
        set.contains(mode)
    }
}
