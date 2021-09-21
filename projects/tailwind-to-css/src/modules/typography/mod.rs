mod decoration;
mod display;
mod font;
mod leading;
mod parser;
mod text;
mod tracking;

pub use self::{decoration::*, text::*};

use crate::{
    css_attributes, syntax_error, CssAttribute, CssBehavior, LengthUnit, Result, TailwindArbitrary, TailwindBuilder,
    TailwindColor, TailwindInstance,
};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

#[derive(Copy, Debug, Clone)]
enum Tracking {
    Normal,
    Length(LengthUnit),
    Global(CssBehavior),
}

#[doc = include_str!("letter-spacing.md")]
#[derive(Copy, Debug, Clone)]
pub struct TailwindTracking {
    kind: Tracking,
}

#[derive(Copy, Debug, Clone)]
enum Leading {
    Normal,
    Length(LengthUnit),
    Global(CssBehavior),
}

#[doc = include_str!("line-height.md")]
#[derive(Copy, Debug, Clone)]
pub struct TailwindLeading {
    kind: Leading,
}

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

#[doc = include_str!("text-underline-offset.md")]
#[derive(Debug, Clone)]
pub enum TailwindUnderlineOffset {
    Auto,
    Unit(usize),
}

#[doc = include_str!("text-overflow.md")]
#[derive(Debug, Clone)]
pub enum TailwindTextOverflow {
    Truncate,
    Ellipsis,
    Clip,
}

#[doc = include_str!("text-indent.md")]
#[derive(Debug, Clone)]
pub enum TailwindIndent {
    Px(f32),
    Unit(f32),
    Percent(f32),
}

#[doc = include_str!("vertical-align.md")]
#[derive(Debug, Clone)]
pub enum TailwindAlign {
    Baseline,
    Top,
    Middle,
    Bottom,
    TextTop,
    TextBottom,
    Sub,
    Super,
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
