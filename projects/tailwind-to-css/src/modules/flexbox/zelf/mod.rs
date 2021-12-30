use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindSelf {
    kind: String,
}

impl<T> From<T> for TailwindSelf
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindSelf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "self-{}", self.kind)
    }
}

impl TailwindInstance for TailwindSelf {
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
            "align-self" => align
        }
    }
}

impl TailwindSelf {
    /// https://tailwindcss.com/docs/align-self
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/align-self#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto",
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
            "self-end",
            "self-start",
            "start",
            "stretch",
            "unsafe-center",
            "unset",
        ]);
        set.contains(mode)
    }
}
