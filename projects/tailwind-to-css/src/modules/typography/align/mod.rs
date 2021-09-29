use super::*;

#[doc = include_str!("readme.md")]
#[derive(Debug, Clone)]
pub enum TailwindAlign {
    Baseline,
    Top,
    Middle,
    Bottom,
    TextTop,
    TextBottom,
    Sub,
    Super,
}
