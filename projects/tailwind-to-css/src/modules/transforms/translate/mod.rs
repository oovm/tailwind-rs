use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindTranslate {
    negative: Negative,
    axis: Option<bool>,
    kind: TranslateSize,
}

#[derive(Clone, Debug)]
enum TranslateSize {
    Unit(f32),
    Length(LengthUnit),
    Standard(String),
    Arbitrary(TailwindArbitrary),
}

impl Display for TranslateSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit(n) => write!(f, "{}", n),
            Self::Length(n) if n.is_fraction() => write!(f, "{}", n.get_class()),
            Self::Length(n) => write!(f, "{}", n.get_class_arbitrary()),
            Self::Standard(s) => write!(f, "{}", s),
            Self::Arbitrary(s) => s.write(f),
        }
    }
}

impl Display for TailwindTranslate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.negative.write(f)?;
        match self.axis {
            Some(true) => write!(f, "translate-x-{}", self.kind),
            Some(false) => write!(f, "translate-y-{}", self.kind),
            None => write!(f, "translate-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindTranslate {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let skew = match self.axis {
            Some(true) => format!("translateX({}rem)", self.kind),
            Some(false) => format!("translateY({}rem)", self.kind),
            None => format!("translate({}rem)", self.kind),
        };
        css_attributes! {
            "transform" => skew
        }
    }
}

impl TailwindTranslate {
    /// https://tailwindcss.com/docs/translate
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        match pattern {
            ["x", rest @ ..] => Ok(Self { negative, axis: Some(true), kind: TranslateSize::parse(rest, arbitrary)? }),
            ["y", rest @ ..] => Ok(Self { negative, axis: Some(false), kind: TranslateSize::parse(rest, arbitrary)? }),
            _ => Ok(Self { negative, axis: None, kind: TranslateSize::parse(pattern, arbitrary)? }),
        }
    }
    /// https://tailwindcss.com/docs/translate#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: Option<bool>, negative: Negative) -> Result<Self> {
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
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
    fn maybe_no_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Unit(arbitrary.as_float()?))
    }
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(arbitrary.as_length()?))
    }
}
