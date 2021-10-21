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
    decoration::*,
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
mod display;
mod font;
mod indent;
mod leading;
mod list;
mod parser;
mod text;
mod tracking;
mod underline_offset;

#[doc = include_str!("text-indent.md")]
#[derive(Debug, Clone)]
pub enum TailwindIndent {
    Px(f32),
    Unit(f32),
    Percent(f32),
}

#[doc = include_str!("whitespace.md")]
#[derive(Debug, Clone)]
pub enum TailwindWhiteSpace {
    Normal,
    NoWrap,
    Pre,
    PreLine,
    PreWrap,
}

#[doc = include_str!("word-break.md")]
#[derive(Debug, Clone)]
pub enum TailwindBreak {
    Normal,
    Words,
    All,
}

#[doc = include_str!("content.md")]
#[derive(Debug, Clone)]
pub enum TailwindContentElement {
    None,
}
