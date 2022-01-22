use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindPosition {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindPosition => "position");

impl Display for TailwindPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl TailwindPosition {
    /// https://tailwindcss.com/docs/position
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("position", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/position#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "static", "relative", "absolute", "fixed", "sticky", "inherit", "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
