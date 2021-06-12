mod builder;
mod parser;

use crate::{ColorResolver, TailwindInstance};

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
#[derive(Clone, Debug)]
pub struct TailwindRingColor {
    pub(crate) color: ColorResolver,
}
