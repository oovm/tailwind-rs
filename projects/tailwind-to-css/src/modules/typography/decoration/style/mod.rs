use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindDecorationStyle {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindDecorationStyle => "text-decoration-style");

impl Display for TailwindDecorationStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                "solid" | "double" | "dotted" | "dashed" | "wavy" => write!(f, "decoration-{}", s),
                _ => write!(f, "decoration-style-{}", s),
            },
            StandardValue::Arbitrary(s) => s.write_class(f, "decoration-style-"),
        }
    }
}

impl TailwindDecorationStyle {
    /// <https://tailwindcss.com/docs/object-fit>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("decoration-style", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-style#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set =
            BTreeSet::from_iter(vec!["dashed", "dotted", "double", "inherit", "initial", "revert", "solid", "unset", "wavy"]);
        set.contains(mode)
    }
}
