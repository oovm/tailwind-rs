use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontVariantNumeric {
    kind: FontNumeric,
}

#[derive(Clone, Debug)]
enum FontNumeric {
    Standard(String),
    Arbitrary(String),
}

impl<T> From<T> for TailwindFontVariantNumeric
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: FontNumeric::Standard(kind.into()) }
    }
}

impl Display for FontNumeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standard(s) => write!(f, "{}", s),
            Self::Arbitrary(s) => write!(f, "[{}]", s),
        }
    }
}

impl Display for TailwindFontVariantNumeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            FontNumeric::Standard(s) => match s.as_str() {
                "normal" => write!(f, "normal-nums"),
                "ordinal" | "slashed-zero" | "lining-nums" | "oldstyle-nums" | "proportional-nums" | "tabular-nums"
                | "diagonal-fractions" | "stacked-fractions" => write!(f, "{}", s),
                _ => write!(f, "font-numeric-{}", s),
            },
            FontNumeric::Arbitrary(s) => write!(f, "font-numeric-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindFontVariantNumeric {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let numeric = match &self.kind {
            FontNumeric::Standard(s) => s,
            FontNumeric::Arbitrary(s) => s,
        };
        css_attributes! {
            "font-variant-numeric" => numeric
        }
    }
}

impl TailwindFontVariantNumeric {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: FontNumeric::parse(kind, arbitrary)? })
    }
}

impl FontNumeric {
    /// https://tailwindcss.com/docs/font-variant-numeric
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [s] => {
                debug_assert!(Self::check_valid(s));
                Ok(Self::Standard(s.to_string()))
            },
            [] => Self::parse_arbitrary(arbitrary),
            _ => syntax_error!("Unknown font-numeric instructions: {}", pattern.join("-")),
        }
    }
    /// https://tailwindcss.com/docs/font-variant-numeric
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self::Arbitrary(arbitrary.to_string()))
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/font-variant-numeric#syntax
    pub fn check_valid(mode: &str) -> bool {
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
