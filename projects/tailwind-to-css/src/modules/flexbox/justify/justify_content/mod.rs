use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindJustifyContent {
    kind: JustifyContent,
}

#[derive(Debug, Clone)]
enum JustifyContent {
    Standard(String),
    Arbitrary(String),
}

impl<T> From<T> for TailwindJustifyContent
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: JustifyContent::Standard(kind.into()) }
    }
}

impl Display for TailwindJustifyContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            JustifyContent::Standard(s) => match s.as_str() {
                "flex-start" => write!(f, "justify-start"),
                "flex-end" => write!(f, "justify-end"),
                "center" => write!(f, "justify-center"),
                "space-between" => write!(f, "justify-between"),
                "space-around" => write!(f, "justify-around"),
                "space-evenly" => write!(f, "justify-evenly"),
                _ => write!(f, "justify-content-{}", s),
            },
            JustifyContent::Arbitrary(s) => write!(f, "justify-content-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindJustifyContent {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let cursor = match &self.kind {
            JustifyContent::Standard(s) => s,
            JustifyContent::Arbitrary(s) => s,
        };
        css_attributes! {
            "justify-content" => cursor
        }
    }
}

impl TailwindJustifyContent {
    /// https://tailwindcss.com/docs/justify-content
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: JustifyContent::Standard(s) })
            },
        }
    }
    /// https://tailwindcss.com/docs/justify-content
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self { kind: JustifyContent::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
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
            "space-around",
            "space-between",
            "space-evenly",
            "start",
            "stretch",
            "unset",
        ]);
        set.contains(mode)
    }
}
