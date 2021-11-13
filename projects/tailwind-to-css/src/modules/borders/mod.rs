use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use crate::{
    css_attributes, syntax_error, CssAttribute, LengthUnit, Result, TailwindArbitrary, TailwindBorderCollapse, TailwindBuilder,
    TailwindColor, TailwindInstance,
};

pub use self::{
    border::*,
    divide::{
        divide_color::TailwindDivideColor, divide_style::TailwindDivideStyle, divide_width::TailwindDivideWidth, TailwindDivide,
    },
    outline::*,
    ring::*,
};

mod border;
mod divide;
mod outline;
mod ring;
#[cfg(test)]
mod test;
