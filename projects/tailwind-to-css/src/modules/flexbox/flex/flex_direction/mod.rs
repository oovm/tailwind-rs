use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFlexDirection {
    kind: String,
}

impl<T> From<T> for TailwindFlexDirection
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindFlexDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.kind.as_str();
        match s {
            "row" => write!(f, "flex-row"),
            "row-reverse" => write!(f, "flex-row-reverse"),
            "column" => write!(f, "flex-col"),
            "column-reverse" => write!(f, "flex-col-reverse"),
            _ => write!(f, "flex-direction-{}", s),
        }
    }
}

impl TailwindInstance for TailwindFlexDirection {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "flex-direction" => self.kind
        }
    }
}

impl TailwindFlexDirection {
    /// https://tailwindcss.com/docs/flex-direction
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "column",
            "column-reverse",
            "inherit",
            "initial",
            "revert",
            "row",
            "row-reverse",
            "unset",
        ]);
        set.contains(mode)
    }
}
