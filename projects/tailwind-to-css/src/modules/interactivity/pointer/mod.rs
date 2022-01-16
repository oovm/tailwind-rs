use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindPointerEvents {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindPointerEvents => "pointer-events");

impl Display for TailwindPointerEvents {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "pointer-events-{}", self.kind)
    }
}

impl TailwindPointerEvents {
    /// <https://tailwindcss.com/docs/pointer-events>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("pointer-events", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/pointer-events#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "all",
            "auto",
            "fill",
            "inherit",
            "initial",
            "none",
            "painted",
            "revert",
            "stroke",
            "unset",
            "visible",
            "visibleFill",
            "visiblePainted",
            "visibleStroke",
        ]);
        set.contains(mode)
    }
}
