pub use self::{
    box_shadow::TailwindShadow, mix_blend::TailwindBlend, mix_blend_bg::TailwindBackgroundBlend, opacity::TailwindOpacity,
    shadow_color::TailwindShadowColor,
};
use crate::{
    css_attributes, Backdrop, CssAttributes, NumericValue, Result, StandardValue, TailwindArbitrary, TailwindBuilder,
    TailwindColor, TailwindInstance,
};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};
mod box_shadow;
mod mix_blend;
mod mix_blend_bg;
mod opacity;
mod shadow_color;
