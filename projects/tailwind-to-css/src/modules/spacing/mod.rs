use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter, Write},
};

use crate::{
    css_attributes, syntax_error, CssAttribute, Result, SpacingAxis, TailwindArbitrary, TailwindBuilder, TailwindInstance,
};

use self::size::SpacingSize;
pub use self::{
    margin::TailwindMargin, margin_scroll::TailwindScrollMargin, padding::TailwindPadding,
    padding_scroll::TailwindScrollPadding, space::TailwindSpace, space_reverse::TailwindSpaceReverse,
};

mod margin;
mod margin_scroll;
mod padding;
mod padding_scroll;
mod size;
mod space;
mod space_reverse;
#[cfg(test)]
mod test;
