use super::*;

impl SizeUnit {
    pub fn parse(kind: &[&str], arbitrary: &str) -> Result<Self> {
        match kind {
            [] => Self::parse_arbitrary(arbitrary),
            ["min"] => Ok(Self::Min),
            ["max"] => Ok(Self::Min),
            ["auto"] => Ok(Self::Min),
            ["full"] => Ok(Self::Min),
            ["min"] => Ok(Self::Min),
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
    pub fn parse(kind: &[&str], arbitrary: &str) -> Result<Self> {
        SizeUnit::parse(kind, arbitrary).map(Self::Normal)
    }
    #[inline]
    pub fn parse_max(kind: &[&str], arbitrary: &str) -> Result<Self> {
        SizeUnit::parse(kind, arbitrary).map(Self::Max)
    }
    #[inline]
    pub fn parse_min(kind: &[&str], arbitrary: &str) -> Result<Self> {
        SizeUnit::parse(kind, arbitrary).map(Self::Min)
    }
}

impl TailwindHeight {
    #[inline]
    pub fn parse(kind: &[&str], arbitrary: &str) -> Result<Self> {
        SizeUnit::parse(kind, arbitrary).map(Self::Normal)
    }
    #[inline]
    pub fn parse_max(kind: &[&str], arbitrary: &str) -> Result<Self> {
        SizeUnit::parse(kind, arbitrary).map(Self::Max)
    }
    #[inline]
    pub fn parse_min(kind: &[&str], arbitrary: &str) -> Result<Self> {
        SizeUnit::parse(kind, arbitrary).map(Self::Min)
    }
}
