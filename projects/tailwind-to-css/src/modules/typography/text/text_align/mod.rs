use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindTextAlignment {
    kind: TextAlignment,
}
#[derive(Debug, Clone)]
enum TextAlignment {
    Standard(String),
    Arbitrary(String),
}

impl<T> From<T> for TailwindTextAlignment
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: TextAlignment::Standard(kind.into()) }
    }
}

impl Display for TailwindTextAlignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TextAlignment::Standard(s) => write!(f, "font-align-{}", s),
            TextAlignment::Arbitrary(s) => write!(f, "font-align-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindTextAlignment {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let align = match &self.kind {
            TextAlignment::Standard(s) => s,
            TextAlignment::Arbitrary(s) => s,
        };
        css_attributes! {
            "text-align" => align
        }
    }
}

impl TailwindTextAlignment {
    /// https://tailwindcss.com/docs/text-align
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: TextAlignment::Standard(s) })
            },
        }
    }
    /// https://tailwindcss.com/docs/text-align
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TextAlignment::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/text-align#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "start",
            "end",
            "left",
            "right",
            "center",
            "justify",
            "justify-all",
            "match-parent",
            "inherit",
            "initial",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
