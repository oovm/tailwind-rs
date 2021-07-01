mod builder;
mod display;

use super::*;

#[doc = include_str!("spacing.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindScale {
    scale: usize,
    axis: Option<bool>,
}

pub struct TailwindRotate {
    deg: usize,
}

enum TranslateSize {}

pub struct TailwindTranslate {
    size: TranslateSize,
    axis: Option<bool>,
}

pub struct TailwindSkew {
    scale: usize,
    axis: bool,
}

pub struct TailwindOrigin {
    wrapper: TailwindObjectPosition,
}
