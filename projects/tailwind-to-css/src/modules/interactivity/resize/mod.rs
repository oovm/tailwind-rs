use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindResize {
    kind: String,
}

impl<T> From<T> for TailwindResize
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindResize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.kind.as_str();
        match s {
            "both" => write!(f, "resize"),
            "horizontal" => write!(f, "resize-x"),
            "vertical" => write!(f, "resize-y"),
            _ => write!(f, "resize-{}", s),
        }
    }
}

impl TailwindInstance for TailwindResize {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "resize" => self.kind
        }
    }
}

impl TailwindResize {
    /// https://tailwindcss.com/docs/user-select
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => Ok(Self::from("both")),
            ["x"] => Ok(Self::from("horizontal")),
            ["y"] => Ok(Self::from("vertical")),
            _ => {
                let kind = pattern.join("-");
                debug_assert!(Self::check_valid(&kind));
                Ok(Self { kind })
            },
        }
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/user-select#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "block",
            "both",
            "horizontal",
            "inherit",
            "initial",
            "inline",
            "none",
            "revert",
            "unset",
            "vertical",
        ]);
        set.contains(mode)
    }
}
