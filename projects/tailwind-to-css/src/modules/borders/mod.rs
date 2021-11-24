use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use crate::{
    css_attributes, syntax_error, CssAttribute, LengthUnit, Result, TailwindArbitrary, TailwindBorderCollapse, TailwindBuilder,
    TailwindColor, TailwindInstance,
};

pub use self::{
    border::{
        border_color::TailwindBorderColor, border_radius::TailwindRounded, border_style::TailwindBorderStyle,
        border_width::TailwindBorderWidth, TailwindBorder,
    },
    divide::{
        divide_color::TailwindDivideColor, divide_reverse::TailwindDivideReverse, divide_style::TailwindDivideStyle,
        divide_width::TailwindDivideWidth, TailwindDivide,
    },
    outline::{
        outline_offset::TailwindOutlineOffset, outline_style::TailwindOutlineStyle, outline_width::TailwindOutlineWidth,
        TailwindOutline,
    },
    ring::{
        ring_color::TailwindRingColor, ring_inset::TailwindRingInset, ring_offset_color::TailwindRingOffsetColor,
        ring_offset_width::TailwindRingOffsetWidth, ring_width::TailwindRingWidth, TailwindRing,
    },
};

mod border;
mod divide;
mod outline;
mod ring;
#[cfg(test)]
mod test;
