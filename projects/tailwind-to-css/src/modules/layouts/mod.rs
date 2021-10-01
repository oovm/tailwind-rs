use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
};

use crate::{
    css_attributes, syntax_error, AnchorPoint, CssAttribute, CssBehavior, LengthUnit, Result, TailwindArbitrary,
    TailwindBuilder, TailwindInstance,
};

pub use self::{
    aspect_ratio::TailwindAspect, columns::TailwindColumns, container::TailwindContainer, object::*,
    position::TailwindPosition, visible::TailwindVisibility, z_index::TailWindZIndex,
};

mod aspect_ratio;
mod boxing;
mod breaking;
mod clear;
mod columns;
mod container;
mod display;
mod float;
mod isolate;
mod object;
mod overflow;
mod overscroll;
mod position;
#[cfg(test)]
mod test;
mod visible;
mod z_index;
