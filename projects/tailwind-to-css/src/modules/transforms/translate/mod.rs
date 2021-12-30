use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindTranslate {
    negative: Negative,
    axis: AxisXY,
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
            AxisXY::X => write!(f, "translate-x-{}", self.kind),
            AxisXY::Y => write!(f, "translate-y-{}", self.kind),
            AxisXY::N => write!(f, "translate-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindTranslate {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let skew = match self.axis {
            AxisXY::X => format!("translateX({}rem)", self.kind),
            AxisXY::Y => format!("translateY({}rem)", self.kind),
            AxisXY::N => format!("translate({}rem)", self.kind),
        };
        css_attributes! {
            "transform" => skew
        }
    }
}

// noinspection DuplicatedCode
impl TailwindTranslate {
    /// <https://tailwindcss.com/docs/translate>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let (rest, axis) = match pattern {
            ["x", rest @ ..] => (rest, AxisXY::X),
            ["y", rest @ ..] => (rest, AxisXY::Y),
            [..] => (pattern, AxisXY::N),
        };
        let kind = TranslateSize::parse(rest, arbitrary)?;
        Ok(Self { negative, axis, kind })
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
