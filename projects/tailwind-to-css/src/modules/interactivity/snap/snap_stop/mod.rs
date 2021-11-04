use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindSnapStop {
    kind: String,
}

impl<T> From<T> for TailwindSnapStop
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindSnapStop {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.kind.as_str();
        match self.kind.as_str() {
            "normal" => write!(f, "snap-normal"),
            "always" => write!(f, "snap-always"),
            _ => write!(f, "snap-stop-{}", s),
        }
    }
}

impl TailwindInstance for TailwindSnapStop {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "scroll-snap-stop" => self.kind
        }
    }
}

impl TailwindSnapStop {
    /// https://tailwindcss.com/docs/scroll-snap-stop
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after snap-stop");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/scroll-snap-stop#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["always", "inherit", "initial", "normal", "unset"]);
        set.contains(mode)
    }
}
