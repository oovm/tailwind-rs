use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindContentAlign {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindContentAlign => "align-content");

impl Display for TailwindContentAlign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "content-align-{}", self.kind)
    }
}

impl TailwindContentAlign {
    /// https://tailwindcss.com/docs/align-content
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("content-align", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/align-content#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "baseline",
            "center",
            "end",
            "first-baseline",
            "flex-end",
            "flex-start",
            "inherit",
            "initial",
            "last-baseline",
            "normal",
            "revert",
            "safe-center",
            "space-around",
            "space-between",
            "space-evenly",
            "start",
            "stretch",
            "unsafe-center",
            "unset",
        ]);
        set.contains(mode)
    }
}
