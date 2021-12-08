use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindTranslate {
    negative: bool,
    axis: Option<bool>,
    kind: TranslateSize,
}

#[derive(Clone, Debug)]
enum TranslateSize {
    Unit(f32),
    Length(LengthUnit),
    Standard(String),
    Arbitrary(String),
}

impl Display for TranslateSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit(n) => write!(f, "{}", n),
            Self::Length(n) if n.is_fraction() => write!(f, "{}", n.get_class()),
            Self::Length(n) => write!(f, "{}", n.get_class_arbitrary()),
            Self::Standard(g) => write!(f, "{}", g),
            Self::Arbitrary(g) => write!(f, "[{}]", g),
        }
    }
}

impl Display for TailwindTranslate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        match self.axis {
            Some(true) => write!(f, "translate-x-{}", self.kind),
            Some(false) => write!(f, "translate-y-{}", self.kind),
            None => write!(f, "translate-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindTranslate {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let skew = match self.axis {
            Some(true) => format!("translateX({}deg)", self.kind),
            Some(false) => format!("translateY({}deg)", self.kind),
            None => format!("translate({}deg)", self.kind),
        };
        css_attributes! {
            "transform" => skew
        }
    }
}

impl TailwindTranslate {
    /// https://tailwindcss.com/docs/translate
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: bool) -> Result<Self> {
        match pattern {
            ["x", rest @ ..] => Ok(Self { negative, axis: Some(true), kind: TranslateSize::parse(rest, arbitrary)? }),
            ["y", rest @ ..] => Ok(Self { negative, axis: Some(false), kind: TranslateSize::parse(rest, arbitrary)? }),
            _ => Ok(Self { negative, axis: None, kind: TranslateSize::parse(pattern, arbitrary)? }),
        }
    }
    /// https://tailwindcss.com/docs/translate#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: Option<bool>, negative: bool) -> Result<Self> {
        Ok(Self { negative, axis, kind: TranslateSize::parse_arbitrary(arbitrary)? })
    }
}

impl TranslateSize {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            ["px"] => Ok(Self::Length(LengthUnit::px(1.0))),
            [s @ ("none" | "inherit" | "initial" | "revert" | "unset")] => Ok(Self::Standard(s.to_string())),
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Self::maybe_no_unit(&a).or_else(|_| Self::maybe_length(&a))
            },
            _ => syntax_error!("Unknown translate instructions: {}", pattern.join("-")),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self::Arbitrary(arbitrary.to_string()))
    }
    fn maybe_no_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Unit(arbitrary.as_float()?))
    }
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(arbitrary.as_length()?))
    }
}
