use crate::{Result, TailwindArbitrary, TailwindInstance};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
enum ShadowKind {
    None,
    Small,
    Normal,
    Medium,
    Large,
    ExtraLarge,
    UltraLarge,
    Custom { x: usize, y: usize, alpha: usize },
}

/// https://tailwindcss.com/docs/box-shadow
#[derive(Copy, Clone, Debug)]
pub struct TailwindShadow {
    kind: ShadowKind,
    is_drop: bool,
}

impl Display for TailwindShadow {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindShadow {}

impl TailwindShadow {
    pub fn parse_box(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
    pub fn parse_drop(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
