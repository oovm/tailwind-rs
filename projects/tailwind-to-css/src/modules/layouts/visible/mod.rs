use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindVisibility {
    kind: String,
}

impl<T> From<T> for TailwindVisibility
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindVisibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.kind.as_str();
        match s {
            "visible" => write!(f, "visible"),
            "hidden" => write!(f, "invisible"),
            _ => write!(f, "visible-{}", s),
        }
    }
}

impl TailwindInstance for TailwindVisibility {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "visibility" => self.kind
        }
    }
}

impl TailwindVisibility {
    /// https://tailwindcss.com/docs/visibility
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after visible");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/visibility#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["collapse", "hidden", "inherit", "initial", "revert", "unset", "visible"]);
        set.contains(mode)
    }
}
