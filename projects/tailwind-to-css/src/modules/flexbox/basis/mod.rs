use super::*;

#[derive(Debug, Copy, Clone)]
enum BasisSize {
    Auto,
    Full,
    Unit(usize),
    Fraction(usize, usize),
    Length(LengthUnit),
}

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindBasis {
    size: BasisSize,
}

impl Display for BasisSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for TailwindBasis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindBasis {}

impl TailwindBasis {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
