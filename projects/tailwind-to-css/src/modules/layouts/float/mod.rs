use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindFloat {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindFloat => "float");

impl Display for TailwindFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "float-{}", self.kind)
    }
}

impl TailwindFloat {
    /// <https://tailwindcss.com/docs/float>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("float", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/float#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "inherit",
            "initial",
            "inline-end",
            "inline-start",
            "left",
            "none",
            "revert",
            "right",
            "unset",
        ]);
        set.contains(mode)
    }
}
