mod builder;
mod display;

use super::*;
use css_style::unit::Rem;

#[doc = include_str ! ("font-family.md")]
#[derive(Debug, Clone)]
pub enum TailwindFontFamily {
    Sans,
    Serif,
    Mono,
}

#[doc = include_str ! ("font-family.md")]
#[derive(Copy, Debug, Clone)]
pub struct TailwindFontSize {
    size: Rem,
    height: Rem,
}

#[doc = include_str ! ("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindFontSmoothing {
    Normal,
    Subpixel,
}

#[doc = include_str ! ("font-smoothing.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontWeight {
    weight: usize,
}

#[doc = include_str ! ("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindTextAlignment {
    Left,
    Center,
    Right,
    Justify,
}
