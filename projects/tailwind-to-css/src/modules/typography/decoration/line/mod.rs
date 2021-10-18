use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindDecorationLine {
    kind: DecorationLine,
}

#[derive(Debug, Clone)]
enum DecorationLine {
    Standard(String),
    Arbitrary(String),
}

impl<T> From<T> for TailwindDecorationLine
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: DecorationLine::Standard(kind.into()) }
    }
}

impl Display for TailwindDecorationLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DecorationLine::Standard(s) => match s.as_str() {
                "underline" | "overline" | "line-through" => write!(f, "{}", s),
                "none" => write!(f, "no-underline"),
                _ => write!(f, "decoration-line-{}", s),
            },
            DecorationLine::Arbitrary(s) => write!(f, "decoration-line-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindDecorationLine {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let cursor = match &self.kind {
            DecorationLine::Standard(s) => s,
            DecorationLine::Arbitrary(s) => s,
        };
        css_attributes! {
            "text-decoration-line" => cursor
        }
    }
}

impl TailwindDecorationLine {
    /// https://tailwindcss.com/docs/cursor
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: DecorationLine::Standard(s) })
            },
        }
    }
    /// https://tailwindcss.com/docs/cursor#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self { kind: DecorationLine::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set =
            BTreeSet::from_iter(vec!["blink", "inherit", "initial", "line-through", "none", "overline", "underline", "unset"]);
        set.contains(mode)
    }
}
