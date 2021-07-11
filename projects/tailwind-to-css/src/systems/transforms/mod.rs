mod display;
mod parser;
use super::*;

#[doc = include_str!("scale.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindScale {
    neg: bool,
    scale: usize,
    axis: Option<bool>,
}

#[derive(Copy, Clone, Debug)]
enum RotateKind {
    This,
    Hue,
    Backdrop,
}

#[doc = include_str!("rotate.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindRotate {
    kind: RotateKind,
    neg: bool,
    deg: usize,
}

#[derive(Copy, Clone, Debug)]
enum TranslateSize {}

#[doc = include_str!("translate.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindTranslate {
    size: TranslateSize,
    axis: Option<bool>,
}

#[doc = include_str!("skew.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindSkew {
    neg: bool,
    deg: usize,
    axis: bool,
}

#[doc = include_str!("transform-origin.md")]
#[derive(Clone, Debug)]
pub struct TailwindOrigin {
    wrapper: TailwindObjectPosition,
}
