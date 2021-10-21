use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindListStyle {
    kind: ListType,
}

#[derive(Debug, Clone)]
enum ListType {
    Standard(String),
    Arbitrary(String),
}

impl<T> From<T> for TailwindListStyle
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: ListType::Standard(kind.into()) }
    }
}

impl Display for TailwindListStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListType::Standard(s) => match s.as_str() {
                "none" | "disc" | "decimal" => write!(f, "list-{}", s),
                _ => write!(f, "list-style-{}", s),
            },
            ListType::Arbitrary(s) => write!(f, "list-type-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindListStyle {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let cursor = match &self.kind {
            ListType::Standard(s) => s,
            ListType::Arbitrary(s) => s,
        };
        css_attributes! {
            "list-style-type" => cursor
        }
    }
}

impl TailwindListStyle {
    /// https://tailwindcss.com/docs/list-style-type
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => {
                debug_assert!(arbitrary.is_some());
                Self::parse_arbitrary(arbitrary)
            },
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: ListType::Standard(s) })
            },
        }
    }
    /// https://tailwindcss.com/docs/list-style-type#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: ListType::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/list-style-type#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "circle",
            "decimal",
            "disc",
            "georgian",
            "inherit",
            "initial",
            "kannada",
            "none",
            "square",
            "trad-chinese-informal",
            "unset",
        ]);
        set.contains(mode)
    }
}
