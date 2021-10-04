use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBreakBefore {
    kind: String,
}

impl<T> From<T> for TailwindBreakBefore
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindBreakBefore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "break-before-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBreakBefore {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "break-before" => self.kind
        }
    }
}

impl TailwindBreakBefore {
    /// https://tailwindcss.com/docs/break-before
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after break-before");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/break-before#syntax
    pub fn check_valid(mode: &str) -> bool {
        TailwindBreakAfter::check_valid(mode)
    }
}
