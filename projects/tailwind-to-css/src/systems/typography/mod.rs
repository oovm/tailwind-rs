mod builder;
mod display;
use super::*;

#[doc = include_str ! ("font-family.md")]
#[derive(Debug, Clone)]
pub enum TailwindFontFamily {}

#[doc = include_str ! ("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindFontSmoothing {
    Normal,
    Subpixel,
}
