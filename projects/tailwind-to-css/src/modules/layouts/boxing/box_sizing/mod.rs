use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBoxSizing {
    kind: String,
}

impl<T> From<T> for TailwindBoxSizing
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindBoxSizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.kind.as_str() {
            "border-box" => write!(f, "box-border"),
            "content-box" => write!(f, "box-content"),
            _ => write!(f, "box-sizing-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindBoxSizing {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "box-sizing" => self.kind
        }
    }
}

impl TailwindBoxSizing {
    /// <https://tailwindcss.com/docs/box-sizing>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after box-sizing");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/box-sizing#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            // Keyword values
            "border-box",
            "content-box",
            // Global values
            "inherit",
            "initial",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
