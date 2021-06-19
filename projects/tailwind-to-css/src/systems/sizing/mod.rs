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

/// used to express sizing
#[derive(Copy, Clone, Debug)]
enum SizeUnit {
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

/// https://tailwindcss.com/docs/height
#[derive(Copy, Clone, Debug)]
pub enum TailwindSizingKind {
    #[doc = include_str!("min-height.md")]
    Width,
    #[doc = include_str!("min-height.md")]
    MinWidth,
    #[doc = include_str!("min-height.md")]
    MaxWidth,
    #[doc = include_str!("min-height.md")]
    Height,
    #[doc = include_str!("min-height.md")]
    MinHeight,
    #[doc = include_str!("min-height.md")]
    MaxHeight,
}

/// https://tailwindcss.com/docs/height
#[derive(Copy, Clone, Debug)]
pub struct TailwindSizing {
    kind: TailwindSizingKind,
    size: SizeUnit,
}
