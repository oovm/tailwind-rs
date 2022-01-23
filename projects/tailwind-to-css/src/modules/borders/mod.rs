pub(crate) use self::outline::outline_adaptor;
pub use self::{
    border::{
        border_color::TailwindBorderColor, border_radius::TailwindRounded, border_style::TailwindBorderStyle,
        border_width::TailwindBorderWidth,
    },
    divide::{
        divide_color::TailwindDivideColor, divide_reverse::TailwindDivideReverse, divide_style::TailwindDivideStyle,
        divide_width::TailwindDivideWidth, TailwindDivide,
    },
    outline::{
        outline_color::TailwindOutlineColor, outline_offset::TailwindOutlineOffset, outline_style::TailwindOutlineStyle,
        outline_width::TailwindOutlineWidth,
    },
    ring::{
        ring_color::TailwindRingColor, ring_inset::TailwindRingInset, ring_offset_color::TailwindRingOffsetColor,
        ring_offset_width::TailwindRingOffsetWidth, ring_width::TailwindRingWidth, TailwindRing,
    },
};
use crate::{
    css_attributes, syntax_error, AxisXY, CssAttributes, LengthUnit, NumericValue, Result, TailwindArbitrary, TailwindBuilder,
    TailwindColor, TailwindInstance, UnitValue,
};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

mod border;
mod divide;
mod outline;
mod ring;
