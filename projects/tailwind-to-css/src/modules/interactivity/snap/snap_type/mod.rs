use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindSnapType {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindSnapType => "scroll-snap-type");

impl Display for TailwindSnapType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "snap-{}", self.kind)
    }
}

impl TailwindSnapType {
    /// <https://tailwindcss.com/docs/scroll-snap-type>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("scroll-type", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/scroll-snap-type#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["block", "both", "inherit", "initial", "inline", "none", "unset", "x", "y"]);
        set.contains(mode)
    }
}
