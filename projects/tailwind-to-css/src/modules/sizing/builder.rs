use super::*;

impl SizingUnit {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match kind {
            ["min"] => Ok(Self::Min),
            ["max"] => Ok(Self::Max),
            ["auto"] => Ok(Self::Auto),
            ["full"] => Ok(Self::Full),
            ["0"] => Self::px(0.0),
            ["px"] => Self::px(1.0),
            [n] => Self::parse_arbitrary(&TailwindArbitrary::from(*n)),
            [] => Self::parse_arbitrary(arbitrary),
            _ => syntax_error!("Unknown sizing instructions: {}", kind.join("-")),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Self::maybe_fraction(arbitrary).or_else(|_| Self::maybe_no_unit(arbitrary)).or_else(|_| Self::maybe_length(arbitrary))
    }
    #[inline]
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(arbitrary.as_length()?))
    }
    #[inline]
    fn maybe_no_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Self::rem(arbitrary.as_float()? / 4.0)
    }
    #[inline]
    fn maybe_fraction(arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (a, b) = arbitrary.as_fraction()?;
        Ok(Self::Fraction(a, b))
    }
    #[inline(always)]
    fn rem(x: f32) -> Result<Self> {
        Ok(Self::Length(LengthUnit::Em(x)))
    }
    #[inline(always)]
    fn px(x: f32) -> Result<Self> {
        Ok(Self::Length(LengthUnit::Px(x)))
    }
}

impl TailwindSizing {
    #[inline]
    pub fn parse_width(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::Width, size: SizingUnit::parse(kind, arbitrary)? })
    }
    #[inline]
    pub fn parse_width_max(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MaxWidth, size: SizingUnit::parse(kind, arbitrary)? })
    }
    #[inline]
    pub fn parse_width_min(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MinWidth, size: SizingUnit::parse(kind, arbitrary)? })
    }
    #[inline]
    pub fn parse_height(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::Height, size: SizingUnit::parse(kind, arbitrary)? })
    }
    #[inline]
    pub fn parse_height_max(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MaxHeight, size: SizingUnit::parse(kind, arbitrary)? })
    }
    #[inline]
    pub fn parse_height_min(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MinHeight, size: SizingUnit::parse(kind, arbitrary)? })
    }
}
