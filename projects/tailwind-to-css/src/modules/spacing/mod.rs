use self::size::Spacing;
pub use self::{margin::TailwindMargin, padding::TailwindPadding, space::TailwindSpace, space_reverse::TailwindSpaceReverse};
use crate::{css_attributes, syntax_error, CssAttribute, Result, TailwindArbitrary, TailwindBuilder, TailwindInstance};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter, Write},
};
mod margin;
mod padding;
mod size;
mod space;
mod space_reverse;
#[cfg(test)]
mod test;
