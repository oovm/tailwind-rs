use crate::{
    css_attributes, syntax_error, CssAttribute, Result, SpacingAxis, TailwindArbitrary, TailwindBuilder, TailwindInstance,
};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter, Write},
};

use self::size::SpacingSize;
pub use self::{margin::TailwindMargin, padding::TailwindPadding, space::TailwindSpace, space_reverse::TailwindSpaceReverse};

mod margin;
mod margin_scroll;
mod padding;
mod padding_scroll;
mod size;
mod space;
mod space_reverse;
#[cfg(test)]
mod test;
