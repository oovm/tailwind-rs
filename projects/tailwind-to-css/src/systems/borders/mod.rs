mod display;
mod parser;

use super::*;

///
#[derive(Copy, Clone, Debug)]
pub enum TailwindBorderStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
    Hidden,
}
///
#[derive(Copy, Clone, Debug)]
pub enum TailwindDivideStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
    // Hidden,
}

///
#[derive(Copy, Clone, Debug)]
pub struct TailwindOutlineWidth {
    width: usize,
}

///
#[derive(Clone, Debug)]
pub struct TailwindOutlineColor {
    pub(crate) color: ColorResolver,
}

///
#[derive(Copy, Clone, Debug)]
pub enum TailwindOutlineStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
    Hidden,
}

///
#[derive(Copy, Clone, Debug)]
pub struct TailwindOutlineOffset {
    offset: usize, // Hidden,
}

///
#[derive(Clone, Debug)]
pub enum TailwindRingWidth {
    Inset,
}

///
#[derive(Clone, Debug)]
pub struct TailwindRingColor {
    pub(crate) color: ColorResolver,
}

///
#[derive(Clone, Debug)]
pub struct TailwindRingOffsetWidth {
    width: usize,
}

///
#[derive(Clone, Debug)]
pub struct TailwindRingOffsetColor {
    pub(crate) color: ColorResolver,
}
