use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindJustifyItems {
    kind: JustifyItems,
}

#[derive(Debug, Clone)]
enum JustifyItems {
    Standard(String),
    Arbitrary(String),
}

impl<T> From<T> for TailwindJustifyItems
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: JustifyItems::Standard(kind.into()) }
    }
}

impl Display for TailwindJustifyItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            JustifyItems::Standard(s) => write!(f, "justify-items-{}", s),
            JustifyItems::Arbitrary(s) => write!(f, "justify-items-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindJustifyItems {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let cursor = match &self.kind {
            JustifyItems::Standard(s) => s,
            JustifyItems::Arbitrary(s) => s,
        };
        css_attributes! {
            "justify-items" => cursor
        }
    }
}

impl TailwindJustifyItems {
    /// https://tailwindcss.com/docs/justify-items
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => {
                debug_assert!(arbitrary.is_some());
                Self::parse_arbitrary(arbitrary)
            },
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: JustifyItems::Standard(s) })
            },
        }
    }
    /// https://tailwindcss.com/docs/justify-items
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: JustifyItems::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "baseline",
            "center",
            "end",
            "flex-end",
            "flex-start",
            "inherit",
            "initial",
            "left",
            "normal",
            "revert",
            "right",
            "self-end",
            "self-start",
            "start",
            "stretch",
            "unset",
        ]);
        set.contains(mode)
    }
}
