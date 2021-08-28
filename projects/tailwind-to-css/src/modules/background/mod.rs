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

pub use self::{
    attachment::TailwindBackgroundAttachment,
    clip::TailwindBackgroundClip,
    color::TailwindBackgroundColor,
    gradient::{TailwindFrom, TailwindTo, TailwindVia},
    origin::TailwindBackgroundOrigin,
    repeat::TailwindBackgroundRepeat,
    size::TailwindBackgroundSize,
};
use crate::{css_attributes, CssAttribute, TailwindBuilder, TailwindColor, TailwindInstance, TailwindObjectPosition};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};
