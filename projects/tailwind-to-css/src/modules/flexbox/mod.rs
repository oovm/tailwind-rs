use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter, Write},
};

use tailwind_ast::parse_integer;

use crate::{
    css_attributes, syntax_error, CssAttribute, CssBehavior, LengthUnit, Result, TailwindArbitrary, TailwindBuilder,
    TailwindInstance,
};

pub use self::{
    basis::TailwindBasis,
    flex::TailwindFlex,
    flex_direction::TailwindFlexDirection,
    flex_wrap::TailwindFlexWrap,
    grow::TailWindGrow,
    order::TailWindOrder,
    place_content::TailwindPlaceContent,
    place_item::TailwindPlaceItems,
    place_self::TailwindPlaceSelf,
    shrink::TailWindShrink,
    span::{TailwindColumn, TailwindRow},
};

mod basis;
mod display;
mod flex;
mod flex_direction;
mod flex_wrap;
mod grow;
mod justify_content;
mod justify_item;
mod justify_self;
mod order;
mod parser;
mod place_content;
mod place_item;
mod place_self;
mod shrink;
mod span;

#[doc=include_str!("grid-template-columns.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindGridTemplate {
    row: bool,
    unit: usize,
}

#[doc=include_str!("grid-auto-flow.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindGridFlow {
    Row,
    RowDense,
    Column,
    ColumnDense,
}

#[derive(Debug, Copy, Clone)]
enum GridAutoKind {
    Auto,
    Min,
    Max,
    Fr,
}

#[doc=include_str!("grid-auto-columns.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindGridAuto {
    kind: GridAutoKind,
    layout: bool,
}

#[derive(Debug, Copy, Clone)]
enum GapSize {
    Px(f32),
    Rem(f32),
}

#[doc=include_str!("gap.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindGap {
    size: GapSize,
    axis: Option<bool>,
}

#[doc=include_str!("align-content.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindContent {
    // Auto,
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}

#[doc=include_str!("align-items.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindItems {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}

#[doc=include_str!("align-self.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindSelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}
