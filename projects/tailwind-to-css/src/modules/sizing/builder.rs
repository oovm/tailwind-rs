use super::*;

impl LengthUnit {
    pub fn parse(kind: &[&str], arbitrary: &str) -> Result<Self> {
        match kind {
            [] => Self::parse_arbitrary(arbitrary),
            ["min"] => Ok(Self::Min),
            ["max"] => Ok(Self::Max),
            ["auto"] => Ok(Self::Auto),
            ["full"] => Ok(Self::Full),
            // px, unit, fract
            _ => Self::parse_arbitrary(arbitrary),
        }
    }
    pub fn parse_arbitrary(_arbitrary: &str) -> Result<Self> {
        todo!()
    }
}

impl TailwindSizing {
    #[inline]
    pub fn parse_width(kind: &[&str], arbitrary: &str) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::Width, size: LengthUnit::parse(kind, arbitrary)? })
    }
    #[inline]
    pub fn parse_width_max(kind: &[&str], arbitrary: &str) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MaxWidth, size: LengthUnit::parse(kind, arbitrary)? })
    }
    #[inline]
    pub fn parse_width_min(kind: &[&str], arbitrary: &str) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MinWidth, size: LengthUnit::parse(kind, arbitrary)? })
    }
    #[inline]
    pub fn parse_height(kind: &[&str], arbitrary: &str) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::Height, size: LengthUnit::parse(kind, arbitrary)? })
    }
    #[inline]
    pub fn parse_height_max(kind: &[&str], arbitrary: &str) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MaxHeight, size: LengthUnit::parse(kind, arbitrary)? })
    }
    #[inline]
    pub fn parse_height_min(kind: &[&str], arbitrary: &str) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MinHeight, size: LengthUnit::parse(kind, arbitrary)? })
    }
}
