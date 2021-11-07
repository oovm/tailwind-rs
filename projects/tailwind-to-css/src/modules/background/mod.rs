use crate::{
    css_attributes, AnchorPoint, CssAttribute, Result, TailwindArbitrary, TailwindBuilder, TailwindColor, TailwindInstance,
};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
};

pub use self::{
    attachment::TailwindBackgroundAttachment,
    clip::TailwindBackgroundClip,
    color::TailwindBackgroundColor,
    gradient::{TailwindFrom, TailwindTo, TailwindVia},
    origin::TailwindBackgroundOrigin,
    repeat::TailwindBackgroundRepeat,
    size::TailwindBackgroundSize,
};

mod attachment;
mod clip;
mod color;
mod gradient;
mod origin;
mod position;
mod repeat;
mod size;
#[cfg(test)]
mod test;
