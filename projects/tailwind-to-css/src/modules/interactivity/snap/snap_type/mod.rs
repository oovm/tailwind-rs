use crate::CssBehavior;

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindSnapType {
    kind: SnapType,
}

#[derive(Debug, Clone)]
enum SnapType {
    None,
    Standard(String),
    Arbitrary(String),
    Global(CssBehavior),
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
            SnapType::None => write!(f, "snap-none"),
            SnapType::Standard(s) => match s.as_str() {
                "start" => write!(f, "snap-start"),
                "end" => write!(f, "snap-end"),
                "center" => write!(f, "snap-center"),
                _ => write!(f, "snap-align-{}", s),
            },
            SnapType::Global(s) => write!(f, "snap-align-{}", s),
            SnapType::Arbitrary(s) => write!(f, "snap-align-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindSnapType {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let cursor = match &self.kind {
            SnapType::None => "none".to_string(),
            SnapType::Standard(s) => s.to_string(),
            SnapType::Arbitrary(s) => s.to_string(),
            SnapType::Global(s) => s.to_string(),
        };
        css_attributes! {
            "scroll-snap-align" => cursor
        }
    }
}

impl TailwindSnapType {
    /// https://tailwindcss.com/docs/cursor
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => {
                debug_assert!(arbitrary.is_some());
                Self::parse_arbitrary(arbitrary)
            },
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: SnapType::Standard(s) })
            },
        }
    }
    /// https://tailwindcss.com/docs/scroll-snap-align
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: SnapType::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/scroll-snap-align#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["block", "both", "inherit", "initial", "inline", "none", "unset", "x", "y"]);
        set.contains(mode)
    }
}
