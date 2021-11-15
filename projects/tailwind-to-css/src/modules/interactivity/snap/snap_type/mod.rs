use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindSnapType {
    kind: SnapType,
}

#[derive(Debug, Clone)]
enum SnapType {
    Standard(String),
    Arbitrary(String),
}

impl<T> From<T> for TailwindSnapType
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: SnapType::Standard(kind.into()) }
    }
}

impl Display for TailwindSnapType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            SnapType::Standard(s) => write!(f, "snap-{}", s),
            SnapType::Arbitrary(s) => write!(f, "snap-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindSnapType {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let cursor = match &self.kind {
            SnapType::Standard(s) => s,
            SnapType::Arbitrary(s) => s,
        };
        css_attributes! {
            "scroll-snap-type" => cursor
        }
    }
}

impl TailwindSnapType {
    /// https://tailwindcss.com/docs/scroll-snap-type
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: SnapType::Standard(s) })
            },
        }
    }
    /// https://tailwindcss.com/docs/scroll-snap-type
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self { kind: SnapType::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/scroll-snap-type#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["block", "both", "inherit", "initial", "inline", "none", "unset", "x", "y"]);
        set.contains(mode)
    }
}
