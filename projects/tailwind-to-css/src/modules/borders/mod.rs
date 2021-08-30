mod border_color;
mod border_radius;
mod border_style;
mod border_width;
mod divide_color;
mod divide_style;
mod divide_width;
mod outline_style;
mod ring_offset_width;
#[cfg(test)]
mod test;

pub use self::{
    border_color::TailwindBorderColor, border_radius::TailwindRounded, border_style::TailwindBorderStyle,
    border_width::TailwindBorderWidth, divide_style::TailwindDivideStyle, outline_style::TailwindOutlineStyle,
    ring_offset_width::TailwindRingOffsetWidth,
};
use crate::{
    css_attributes, syntax_error, CssAttribute, Result, TailwindArbitrary, TailwindBuilder, TailwindColor, TailwindInstance,
};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

#[derive(Copy, Clone, Debug)]
enum BorderStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
    Hidden,
}

impl Display for BorderStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Solid => write!(f, "solid"),
            Self::Dashed => write!(f, "dashed"),
            Self::Dotted => write!(f, "dotted"),
            Self::Double => write!(f, "double"),
            Self::Hidden => write!(f, "hidden"),
            Self::None => write!(f, "none"),
        }
    }
}

///
#[derive(Copy, Clone, Debug)]
pub struct TailwindOutlineWidth {
    width: usize,
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
    pub(crate) color: TailwindColor,
}

///
#[derive(Clone, Debug)]
pub struct TailwindRingOffsetColor {
    pub(crate) color: TailwindColor,
}
