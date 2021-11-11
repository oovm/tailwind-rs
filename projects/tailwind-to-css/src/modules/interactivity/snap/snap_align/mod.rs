use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindSnapAlign {
    kind: SnapAlign,
}

#[derive(Debug, Clone)]
enum SnapAlign {
    Standard(String),
    Arbitrary(String),
}

impl<T> From<T> for TailwindSnapAlign
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: SnapAlign::Standard(kind.into()) }
    }
}

impl Display for TailwindSnapAlign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            SnapAlign::Standard(s) => match s.as_str() {
                "start" => write!(f, "snap-start"),
                "end" => write!(f, "snap-end"),
                "center" => write!(f, "snap-center"),
                _ => write!(f, "snap-align-{}", s),
            },
            SnapAlign::Arbitrary(s) => write!(f, "snap-align-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindSnapAlign {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let cursor = match &self.kind {
            SnapAlign::Standard(s) => s,
            SnapAlign::Arbitrary(s) => s,
        };
        css_attributes! {
            "scroll-snap-align" => cursor
        }
    }
}

impl TailwindSnapAlign {
    /// https://tailwindcss.com/docs/cursor
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: SnapAlign::Standard(s) })
            },
        }
    }
    /// https://tailwindcss.com/docs/scroll-snap-align
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self { kind: SnapAlign::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/scroll-snap-align#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["center", "inherit", "initial", "none", "revert", "unset"]);
        set.contains(mode)
    }
}
