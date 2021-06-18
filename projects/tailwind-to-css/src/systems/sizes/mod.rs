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
pub enum TailwindSizing {
    Min,
    Max,
    Fit,
    Full,
    Auto,
    Screen,
    Px(f32),
    Unit(usize),
    Percent(usize, usize),
}

/// https://tailwindcss.com/docs/height
#[derive(Clone, Debug)]
pub enum TailwindWidth {
    #[doc = include_str!("min-width.md")]
    Min(TailwindSizing),
    #[doc = include_str!("max-width.md")]
    Max(TailwindSizing),
    #[doc = include_str!("width.md")]
    Normal(TailwindSizing),
}

/// https://tailwindcss.com/docs/width
#[derive(Clone, Debug)]
pub enum TailwindHeight {
    #[doc = include_str!("min-height.md")]
    Min(TailwindSizing),
    #[doc = include_str!("max-height.md")]
    Max(TailwindSizing),
    #[doc = include_str!("height.md")]
    Normal(TailwindSizing),
}
