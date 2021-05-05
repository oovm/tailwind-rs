mod builder;
mod parser;

use super::*;

pub struct SizingSystem {}

/// used to express sizing
#[derive(Clone)]
pub enum TailwindSizing {
    Min,
    Max,
    Fit,
    Full,
    Auto,
    Screen,
    Length(Length),
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

impl TailwindSizing {
    /// `w-px`
    pub fn px(n: usize) -> Self {
        Self::Length(px(n as f32))
    }
    /// `w-{number}`
    pub fn number(number: usize) -> Self {
        Self::Length(rem(number as f32 / 4.0))
    }
    /// `w-[{n}rem]`
    pub fn rem(number: usize) -> Self {
        Self::Length(rem(number as f32))
    }
    /// `w-{a}/{b}` & `w-full`
    pub fn percent(numerator: usize, denominator: usize) -> Self {
        assert!(numerator <= denominator);
        if numerator == denominator { Self::Full } else { Self::Percent(numerator, denominator) }
    }
}

impl Debug for TailwindSizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("width: ")?;
        match self {
            Self::Min => f.write_str("min-content")?,
            Self::Max => f.write_str("max-content")?,
            Self::Fit => f.write_str("fit-content")?,
            Self::Screen => f.write_str("fit-content")?,
            Self::Full => f.write_str("fit-content")?,
            Self::Auto => f.write_str("auto")?,
            Self::Length(n) => Display::fmt(n, f)?,
            Self::Percent(numerator, denominator) => Display::fmt(&percent(*numerator as f32 / *denominator as f32), f)?,
        }
        f.write_char(';')
    }
}
