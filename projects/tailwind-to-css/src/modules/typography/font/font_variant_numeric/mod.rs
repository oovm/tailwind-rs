use crate::KeywordOnly;

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontVariantNumeric {
    kind: KeywordOnly,
}

crate::macros::sealed::keyword_instance!(TailwindFontVariantNumeric => "font-variant-numeric");

impl Display for TailwindFontVariantNumeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            KeywordOnly::Standard(s) => match s.as_str() {
                "normal" => write!(f, "normal-nums"),
                "ordinal" | "slashed-zero" | "lining-nums" | "oldstyle-nums" | "proportional-nums" | "tabular-nums"
                | "diagonal-fractions" | "stacked-fractions" => write!(f, "{}", s),
                _ => write!(f, "font-numeric-{}", s),
            },
            KeywordOnly::Arbitrary(s) => write!(f, "font-numeric-[{}]", s),
        }
    }
}

impl TailwindFontVariantNumeric {
    /// https://tailwindcss.com/docs/font-variant-numeric
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parser("font-number", &Self::check_valid)(kind, arbitrary)? })
    }
    /// https://tailwindcss.com/docs/font-variant-numeric#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parse_arbitrary(arbitrary)? })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/font-variant-numeric#syntax
    fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "diagonal-fractions",
            "inherit",
            "initial",
            "lining-nums",
            "normal",
            "oldstyle-nums",
            "ordinal",
            "proportional-nums",
            "slashed-zero",
            "stacked-fractions",
            "tabular-nums",
            "unset",
        ]);
        set.contains(mode)
    }
}
