use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindTextTransform {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindTextTransform => "text-transform");

impl Display for TailwindTextTransform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "text-transform-{}", self.kind)
    }
}

impl TailwindTextTransform {
    /// <https://tailwindcss.com/docs/text-transform>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("text-transform", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-transform#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "none",
            "capitalize",
            "uppercase",
            "lowercase",
            "full-width",
            "full-size-kana",
            "inherit",
            "initial",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
