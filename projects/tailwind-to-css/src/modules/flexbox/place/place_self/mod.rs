use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindPlaceSelf {
    kind: MaybeArbitrary,
}

impl<T> From<T> for TailwindPlaceSelf
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: MaybeArbitrary::Standard(kind.into()) }
    }
}

impl Display for TailwindPlaceSelf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "place-self-{}", self.kind)
    }
}

impl TailwindInstance for TailwindPlaceSelf {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "place-self" => self.kind.get_properties()
        }
    }
}

impl TailwindPlaceSelf {
    /// https://tailwindcss.com/docs/place-items
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => {
                let s = pattern.join("-");
                debug_assert!(Self::check_valid(&s));
                Ok(Self { kind: MaybeArbitrary::Standard(s) })
            },
        }
    }
    /// https://tailwindcss.com/docs/place-items
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self { kind: MaybeArbitrary::Arbitrary(arbitrary.to_string()) })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/place-items#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["center", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
