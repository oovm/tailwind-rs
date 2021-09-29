mod align;
mod decoration;
mod display;
mod font;
mod indent;
mod leading;
mod parser;
mod text;
mod tracking;
mod underline_offset;

pub use self::{
    align::TailwindAlign, decoration::*, font::*, leading::TailwindLeading, text::*, tracking::TailwindTracking,
    underline_offset::TailwindUnderlineOffset,
};

use crate::{
    css_attributes, syntax_error, CssAttribute, CssBehavior, LengthUnit, Result, TailwindArbitrary, TailwindBuilder,
    TailwindColor, TailwindInstance,
};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

#[doc = include_str!("list-style-type.md")]
#[derive(Debug, Clone)]
pub enum TailwindListStyle {
    None,
    Disc,
    Decimal,
    Custom(String),
}

#[derive(Copy, Debug, Clone)]
enum ListStylePosition {
    Inside,
    Outside,
}

#[doc = include_str!("list-style-position.md")]
#[derive(Copy, Debug, Clone)]
pub struct TailwindListStylePosition {
    kind: ListStylePosition,
}

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
