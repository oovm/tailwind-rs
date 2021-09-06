use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use crate::{css_attributes, syntax_error, CssAttribute, Result, TailwindArbitrary, TailwindBuilder, TailwindInstance};

pub use self::{
    box_color::TailwindShadowColor, box_shadow::TailwindShadow, mix_blend::TailwindBlend,
    mix_blend_bg::TailwindBackgroundBlend, opacity::TailwindOpacity,
};

mod box_color;
mod box_shadow;
mod mix_blend;
mod mix_blend_bg;
mod opacity;
#[cfg(test)]
mod test;
