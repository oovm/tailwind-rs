pub use self::{
    box_color::TailwindShadowColor, box_shadow::TailwindShadow, mix_blend::TailwindBlend,
    mix_blend_bg::TailwindBackgroundBlend, opacity::TailwindOpacity,
};
use crate::{
    css_attributes, syntax_error, Backdrop, CssAttributes, NumericValue, Result, TailwindArbitrary, TailwindBuilder,
    TailwindInstance,
};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};
mod box_color;
mod box_shadow;
mod mix_blend;
mod mix_blend_bg;
mod opacity;
