use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindBackgroundOrigin {
    kind: String,
}

impl<T> From<T> for TailwindBackgroundOrigin
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindBackgroundOrigin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.kind.as_str();
        match s {
            "border-box" => write!(f, "bg-origin-border"),
            "padding-box" => write!(f, "bg-origin-padding"),
            "content-box" => write!(f, "bg-origin-content	"),
            _ => write!(f, "bg-origin-{}", s),
        }
    }
}

impl TailwindInstance for TailwindBackgroundOrigin {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "background-origin" => self.kind
        }
    }
}

impl TailwindBackgroundOrigin {
    /// https://tailwindcss.com/docs/background-origin
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after clear");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/background-origin#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set =
            BTreeSet::from_iter(vec!["border-box", "content-box", "inherit", "initial", "padding-box", "revert", "unset"]);
        set.contains(mode)
    }
}
