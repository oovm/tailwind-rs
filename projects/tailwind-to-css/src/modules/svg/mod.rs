pub use self::{
    fill::TailwindFillColor,
    stroke::{stroke_color::TailwindStrokeColor, stroke_width::TailwindStrokeWidth, TailwindStroke},
};
use crate::{css_attributes, CssAttribute, Result, TailwindArbitrary, TailwindBuilder, TailwindColor, TailwindInstance};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};
mod fill;
mod stroke;
