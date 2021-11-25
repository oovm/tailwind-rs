use crate::KeywordOnly;

use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontVariantNumeric {
    kind: KeywordOnly,
}

impl<T> From<T> for TailwindFontVariantNumeric
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: KeywordOnly::Standard(kind.into()) }
    }
}

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

impl TailwindInstance for TailwindFontVariantNumeric {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let numeric = match &self.kind {
            KeywordOnly::Standard(s) => s,
            KeywordOnly::Arbitrary(s) => s,
        };
        css_attributes! {
            "font-variant-numeric" => numeric
        }
    }
}

impl TailwindFontVariantNumeric {
    /// https://tailwindcss.com/docs/font-variant-numeric
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parser("font-number", &check_valid)(kind, arbitrary)? })
    }
    /// https://tailwindcss.com/docs/font-variant-numeric#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parse_arbitrary(arbitrary)? })
    }
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
