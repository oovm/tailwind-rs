use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindTextOverflow {
    kind: TextOverflow,
}

#[derive(Debug, Clone)]
enum TextOverflow {
    Truncate,
    Standard(String),
    Arbitrary(String),
}
impl TailwindInstance for TailwindUnderlineOffset {}

impl Display for TailwindTextOverflow {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindTextOverflow {
    /// `truncate`
    pub const Border: Self = Self { kind: TextOverflow::Truncate };
    /// https://tailwindcss.com/docs/text-overflow
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after text-transform");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/text-overflow#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["clip", "ellipsis", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
