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
/// font that unknown at parsing time
#[derive(Debug, Clone)]
pub enum TailwindFontArbitrary {
    Unsolved,
}

// #[doc = include_str!("font-family.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontFamily {
    name: String,
}

// #[doc = include_str!("font-size.md")]
#[derive(Copy, Debug, Clone)]
pub struct TailwindFontSize {
    size: TailwindTracking,
    height: TailwindLeading,
}

// #[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindFontSmoothing {
    Normal,
    Subpixel,
}

// #[doc = include_str!("font-style.md")]
#[derive(Debug, Clone)]
pub enum TailwindFontStyle {
    Italic,
    Normal,
}

// #[doc = include_str!("font-weight.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontWeight {
    weight: usize,
}

#[derive(Copy, Debug, Clone)]
enum FontVariantNumeric {
    Normal,
    Ordinal,
    SlashedZero,
    Lining,
    OldStyle,
    Proportional,
    Tabular,
    DiagonalFractions,
    StackedFractions,
}

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
