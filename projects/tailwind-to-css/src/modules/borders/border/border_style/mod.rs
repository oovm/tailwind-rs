use crate::StandardValue;

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBorderStyle {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindBorderStyle => "border-style");

impl Display for TailwindBorderStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "border-{}", self.kind)
    }
}

impl TailwindBorderStyle {
    /// https://tailwindcss.com/docs/object-fit
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("border-style", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-style#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "ridge", "inset", "outset", "inherit",
            "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
