use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundClip {
    kind: String,
}

impl<T> From<T> for TailwindBackgroundClip
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindBackgroundClip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-clip-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBackgroundClip {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "background-clip" => self.kind
        }
    }
}

impl TailwindBackgroundClip {
    /// https://tailwindcss.com/docs/background-clip
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after bg-clip");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/background-clip#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "border-box",
            "content-box",
            "inherit",
            "initial",
            "padding-box",
            "revert",
            "text",
            "unset",
        ]);
        set.contains(mode)
    }
}
