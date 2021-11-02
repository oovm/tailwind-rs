use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindWillChange {
    kind: WillChange,
}

#[derive(Debug, Clone)]
pub enum WillChange {
    Standard(String),
    Arbitrary(String),
}

impl<T> From<T> for TailwindWillChange
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: WillChange::Standard(kind.into()) }
    }
}

impl Display for TailwindWillChange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            WillChange::Standard(s) => write!(f, "will-change-{}", s),
            WillChange::Arbitrary(s) => write!(f, "will-change-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindWillChange {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let align = match &self.kind {
            WillChange::Standard(s) => s,
            WillChange::Arbitrary(s) => s,
        };
        css_attributes! {
            "will-change" => align
        }
    }
}

impl TailwindWillChange {
    /// https://tailwindcss.com/docs/will-change
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: WillChange::Standard(s) })
            },
        }
    }
    /// https://tailwindcss.com/docs/will-change#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self { kind: WillChange::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/will-change#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto",
            "contents",
            "inherit",
            "initial",
            "opacity",
            "revert",
            "scroll-position",
            "transform",
            "unset",
        ]);
        set.contains(mode)
    }
}
