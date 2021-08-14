mod builder;
mod display;

use super::*;

#[derive(Copy, Clone, Debug)]
pub enum LengthResolver {
    Px(f32),
    Rem(f32),
    Percent(f32),
    Unit(isize),
    Fraction(usize, usize),
}

#[derive(Copy, Clone, Debug)]
enum LengthUnit {
    Min,
    Max,
    Fit,
    Full,
    Auto,
    Screen,
    Px(f32),
    Rem(f32),
    Unit(usize),
    Fraction(usize, usize),
    Percent(f32),
}

#[derive(Copy, Clone, Debug)]
enum TailwindSizingKind {
    Width,
    MinWidth,
    MaxWidth,
    Height,
    MinHeight,
    MaxHeight,
}

#[doc = include_str!("sizing.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindSizing {
    kind: TailwindSizingKind,
    size: LengthUnit,
}
