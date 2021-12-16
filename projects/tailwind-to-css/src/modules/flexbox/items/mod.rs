use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindItems {
    kind: String,
}

impl<T> From<T> for TailwindItems
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "items-{}", self.kind)
    }
}

impl TailwindInstance for TailwindItems {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let s = self.kind.as_str();
        let align = match s {
            "first-baseline" => "first baseline",
            "last-baseline" => "last baseline",
            "safe-center" => "safe center",
            "unsafe-items" => "unsafe center",
            _ => s,
        };
        css_attributes! {
            "align-content" => align
        }
    }
}

impl TailwindItems {
    /// <https://tailwindcss.com/docs/align-items>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after items");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "baseline",
            "center",
            "end",
            "flex-end",
            "flex-start",
            "inherit",
            "initial",
            "normal",
            "revert",
            "start",
            "stretch",
            "unset",
        ]);
        set.contains(mode)
    }
}
