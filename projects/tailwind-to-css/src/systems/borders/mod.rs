mod builder;
mod parser;

use crate::{ColorResolver, TailwindInstance};

pub enum TailwindBorderStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
    Hidden,
}

pub enum TailwindDivideStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
    // Hidden,
}

pub struct TailwindRingColor {
    pub(crate) color: ColorResolver,
}
