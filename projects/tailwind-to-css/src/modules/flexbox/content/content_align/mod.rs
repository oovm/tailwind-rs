use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindContentAlign {
    kind: String,
}

impl<T> From<T> for TailwindContentAlign
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindContentAlign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "content-align-{}", self.kind)
    }
}

impl TailwindInstance for TailwindContentAlign {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let s = self.kind.as_str();
        let align = match s {
            "first-baseline" => "first baseline",
            "last-baseline" => "last baseline",
            "safe-center" => "safe center",
            "unsafe-center" => "unsafe center",
            _ => s,
        };
        css_attributes! {
            "align-content" => align
        }
    }
}

impl TailwindContentAlign {
    /// https://tailwindcss.com/docs/align-content
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after content");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/align-content#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "baseline",
            "center",
            "end",
            "first-baseline",
            "flex-end",
            "flex-start",
            "inherit",
            "initial",
            "last-baseline",
            "normal",
            "revert",
            "safe-center",
            "space-around",
            "space-between",
            "space-evenly",
            "start",
            "stretch",
            "unsafe-center",
            "unset",
        ]);
        set.contains(mode)
    }
}
