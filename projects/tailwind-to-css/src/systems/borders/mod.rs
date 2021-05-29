mod builder;
mod parser;

use crate::TailwindInstance;

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
