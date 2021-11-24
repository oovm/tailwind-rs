use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
};

use crate::{
    css_attributes, syntax_error, AnchorPoint, CssAttribute, Result, TailwindArbitrary, TailwindBuilder, TailwindColor,
    TailwindInstance,
};

pub use self::{
    attachment::TailwindBackgroundAttachment,
    clip::TailwindBackgroundClip,
    color::TailwindBackgroundColor,
    gradient::{TailwindFrom, TailwindTo, TailwindVia},
    image::TailwindBackgroundImage,
    origin::TailwindBackgroundOrigin,
    position::TailwindBackgroundPosition,
    repeat::TailwindBackgroundRepeat,
    size::TailwindBackgroundSize,
};

mod attachment;
mod clip;
mod color;
mod gradient;
mod image;
mod origin;
mod position;
mod repeat;
mod size;
#[cfg(test)]
mod test;
