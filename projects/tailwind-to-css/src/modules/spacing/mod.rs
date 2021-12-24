use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
};

use crate::{
    css_attributes, syntax_error, CssAttributes, Result, SpacingAxis, TailwindArbitrary, TailwindBuilder, TailwindInstance,
};

use self::size::SpacingSize;
pub use self::{
    margin::TailwindMargin, margin_scroll::TailwindScrollMargin, padding::TailwindPadding,
    padding_scroll::TailwindScrollPadding, space::TailwindSpace, space_reverse::TailwindSpaceReverse,
};
use crate::Negative;
mod margin;
mod margin_scroll;
mod padding;
mod padding_scroll;
mod size;
mod space;
mod space_reverse;
