use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFlexWrap {
    kind: String,
}

impl<T> From<T> for TailwindFlexWrap
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindFlexWrap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.kind.as_str();
        match s {
            "wrap" => write!(f, "flex-wrap"),
            "wrap-reverse" => write!(f, "flex-wrap-reverse"),
            "nowrap" => write!(f, "flex-nowrap"),
            _ => write!(f, "flex-wrap-{}", s),
        }
    }
}

impl TailwindInstance for TailwindFlexWrap {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "flex-wrap" => self.kind
        }
    }
}

impl TailwindFlexWrap {
    /// https://tailwindcss.com/docs/flex-wrap
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after flex-wrap");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/flex-wrap#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "nowrap", "revert", "unset", "wrap", "wrap-reverse"]);
        set.contains(mode)
    }
}
