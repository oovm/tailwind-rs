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
use crate::{
    css_attributes, AnchorPoint, CssAttributes, Result, StandardValue, TailwindArbitrary, TailwindBuilder, TailwindColor, TailwindInstance,
};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
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
