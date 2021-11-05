use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use crate::{
    css_attributes, syntax_error, CssAttribute, CssBehavior, LengthUnit, Result, TailwindArbitrary, TailwindBuilder,
    TailwindColor, TailwindInstance,
};

pub use self::{
    align::TailwindAlign,
    decoration::{
        color::TailwindDecorationColor, line::TailwindDecorationLine, style::TailwindDecorationStyle,
        thickness::TailwindDecorationThickness, TailwindDecoration,
    },
    font::*,
    leading::TailwindLeading,
    list::{list_position::TailwindListPosition, list_type::TailwindListStyle, TailwindList},
    text::*,
    tracking::TailwindTracking,
    underline_offset::TailwindUnderlineOffset,
};

mod align;
mod breaking;
mod decoration;
mod font;
mod indent;
mod leading;
mod list;
mod text;
mod tracking;
mod underline_offset;
mod whitespace;
