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
    gap::TailwindGap,
    grid::{grid_auto::TailwindGridAuto, grid_flow::TailwindGridFlow, grid_rows::TailwindGridRows, TailwindGrid},
    grow::TailWindGrow,
    justify_content::TailwindJustifyContent,
    justify_item::TailwindJustifyItems,
    justify_self::TailwindJustifySelf,
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
mod gap;
mod grid;
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
