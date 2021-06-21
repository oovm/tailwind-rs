mod display;
mod parser;

use super::*;

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
    size: f32,
    height: f32,
}

#[doc = include_str ! ("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindFontSmoothing {
    Normal,
    Subpixel,
}

#[doc = include_str ! ("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindFontStyle {
    Italic,
    Normal,
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

#[doc = include_str ! ("font-smoothing.md")]
#[derive(Debug, Clone)]
pub struct TailwindTextColor {
    pub(crate) color: ColorResolver,
}

#[doc = include_str ! ("font-smoothing.md")]
#[derive(Debug, Clone)]
pub struct TailwindUnderlineOffset {}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindTracking {
    Tighter,
    Tight,
    Normal,
    Wide,
    Wider,
    Widest,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindLeading {}

#[doc = include_str ! ("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindContent {
    None,
}
