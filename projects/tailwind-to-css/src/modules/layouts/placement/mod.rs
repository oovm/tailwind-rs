use super::*;

pub(crate) mod bottom;
pub(crate) mod inset;
pub(crate) mod left;
pub(crate) mod right;
pub(crate) mod top;

#[derive(Debug, Clone)]
enum PlacementSize {
    Auto,
    Full,
    // Unit(f32),
    Fraction(usize, usize),
    Length(LengthUnit),
    Global(CssBehavior),
    Arbitrary(String),
}

impl Display for PlacementSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Full => write!(f, "full"),
            // Self::Unit(n) => write!(f, "{}", n),
            Self::Fraction(a, b) => write!(f, "{}/{}", a, b),
            Self::Length(n) => write!(f, "{}", n.get_class_arbitrary()),
            Self::Global(g) => write!(f, "{}", g),
            Self::Arbitrary(a) => write!(f, "[{}]", a),
        }
    }
}

impl PlacementSize {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after z-index");
        let kind = match kind {
            [] => Self::parse_arbitrary(arbitrary)?,
            ["px"] => Self::Length(LengthUnit::px(1.0)),
            ["full"] => Self::Full,
            ["auto"] => Self::Auto,
            ["inherit"] => Self::Global(CssBehavior::Inherit),
            ["initial"] => Self::Global(CssBehavior::Initial),
            ["revert"] => Self::Global(CssBehavior::Revert),
            ["unset"] => Self::Global(CssBehavior::Unset),
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Self::parse_arbitrary(&a)?
            },
            _ => return syntax_error!("Unknown placement instructions"),
        };
        Ok(kind)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Self::maybe_fraction(arbitrary).or_else(|_| Self::maybe_no_unit(arbitrary)).or_else(|_| Self::maybe_length(arbitrary))
    }
    #[inline]
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(arbitrary.as_length()?))
    }
    #[inline]
    fn maybe_no_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(LengthUnit::rem(arbitrary.as_float()? / 4.0)))
    }
    #[inline]
    fn maybe_fraction(arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (a, b) = arbitrary.as_fraction()?;
        Ok(Self::Fraction(a, b))
    }
}
