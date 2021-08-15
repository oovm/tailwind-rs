mod builder;
mod display;

use super::*;

#[derive(Copy, Clone, Debug)]
enum TailwindSizingKind {
    Width,
    MinWidth,
    MaxWidth,
    Height,
    MinHeight,
    MaxHeight,
}

#[derive(Copy, Clone, Debug)]
enum SizingUnit {
    Min,
    Max,
    Fit,
    Auto,
    Full,
    Screen,
    Fraction(usize, usize),
    Length(LengthUnit),
}

#[doc = include_str!("sizing.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindSizing {
    kind: TailwindSizingKind,
    size: SizingUnit,
}
