use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindPlaceSelf {
    kind: PlaceItems,
}

#[derive(Debug, Clone)]
enum PlaceItems {
    Standard(String),
    Arbitrary(String),
}

impl<T> From<T> for TailwindPlaceSelf
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: PlaceItems::Standard(kind.into()) }
    }
}

impl Display for TailwindPlaceSelf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            PlaceItems::Standard(s) => write!(f, "place-items-{}", s),
            PlaceItems::Arbitrary(s) => write!(f, "place-items-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindPlaceSelf {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let cursor = match &self.kind {
            PlaceItems::Standard(s) => s,
            PlaceItems::Arbitrary(s) => s,
        };
        css_attributes! {
            "place-items" => cursor
        }
    }
}

impl TailwindPlaceSelf {
    /// https://tailwindcss.com/docs/place-items
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => {
                debug_assert!(arbitrary.is_some());
                Self::parse_arbitrary(arbitrary)
            },
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: PlaceItems::Standard(s) })
            },
        }
    }
    /// https://tailwindcss.com/docs/place-items
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: PlaceItems::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/place-items#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["center", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
