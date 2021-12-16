use super::*;

#[derive(Debug, Copy, Clone)]
enum BasisSize {
    Auto,
    Fill,
    Max,
    Min,
    Fit,
    Content,
    Full,
    Unit(usize),
    Length(LengthUnit),
    Global(CssBehavior),
}

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindBasis {
    kind: BasisSize,
}

impl Display for BasisSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Full => write!(f, "full"),
            Self::Fill => write!(f, "fill"),
            Self::Max => write!(f, "max"),
            Self::Min => write!(f, "min"),
            Self::Fit => write!(f, "fit"),
            Self::Content => write!(f, "content"),
            Self::Unit(n) => write!(f, "{}", n),
            Self::Length(n) if n.is_fraction() => write!(f, "{}", n.get_class()),
            Self::Length(n) => write!(f, "{}", n.get_class_arbitrary()),
            Self::Global(g) => write!(f, "{}", g),
        }
    }
}

impl Display for TailwindBasis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "basis-{}", self.kind)
    }
}

impl BasisSize {
    pub fn get_properties(&self) -> String {
        match self {
            BasisSize::Auto => "auto".to_string(),
            BasisSize::Fill => "fill".to_string(),
            BasisSize::Max => "max-content".to_string(),
            BasisSize::Min => "min-content".to_string(),
            BasisSize::Fit => "fit-content".to_string(),
            BasisSize::Content => "content".to_string(),
            BasisSize::Full => "100%".to_string(),
            BasisSize::Unit(n) => format!("{}rem", *n as f32 / 4.0),
            BasisSize::Length(n) => n.get_properties(),
            BasisSize::Global(g) => g.to_string(),
        }
    }
}

impl TailwindInstance for TailwindBasis {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "flex-basis" => self.kind.get_properties()
        }
    }
}

impl TailwindBasis {
    /// <https://tailwindcss.com/docs/basis>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            ["auto"] => Ok(Self { kind: BasisSize::Auto }),
            ["fill"] => Ok(Self { kind: BasisSize::Fill }),
            ["min"] => Ok(Self { kind: BasisSize::Min }),
            ["max"] => Ok(Self { kind: BasisSize::Max }),
            ["fit"] => Ok(Self { kind: BasisSize::Fit }),
            ["content"] => Ok(Self { kind: BasisSize::Content }),
            ["inherit"] => Ok(Self { kind: BasisSize::Global(CssBehavior::Inherit) }),
            ["initial"] => Ok(Self { kind: BasisSize::Global(CssBehavior::Initial) }),
            ["unset"] => Ok(Self { kind: BasisSize::Global(CssBehavior::Unset) }),
            ["full"] => Ok(Self { kind: BasisSize::Full }),
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Self::maybe_frac(&a).or_else(|_| Self::maybe_unit(&a))
            },
            [] => Self::parse_arbitrary(arbitrary),
            _ => syntax_error!("Unknown basis instructions"),
        }
    }

    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Self::maybe_length(arbitrary).or_else(|_| Self::maybe_frac(arbitrary)).or_else(|_| Self::maybe_unit(arbitrary))
    }
    fn maybe_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: BasisSize::Unit(arbitrary.as_integer()?) })
    }
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: BasisSize::Length(arbitrary.as_length()?) })
    }
    fn maybe_frac(arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (a, b) = arbitrary.as_fraction()?;
        Ok(Self { kind: BasisSize::Length(LengthUnit::Fraction(a, b)) })
    }
}
