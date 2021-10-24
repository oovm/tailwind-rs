pub use self::{
    basis::TailwindBasis,
    flex::{flex_adaptor, flex_direction::TailwindFlexDirection, flex_wrap::TailwindFlexWrap, TailwindFlex},
    gap::TailwindGap,
    grid::{
        grid_auto::TailwindGridAuto, grid_cols::TailwindGridColumns, grid_flow::TailwindGridFlow, grid_rows::TailwindGridRows,
        TailwindGrid,
    },
    grow::TailWindGrow,
    justify::{justify_content::TailwindJustifyContent, justify_item::TailwindJustifyItems, justify_self::TailwindJustifySelf},
    order::TailWindOrder,
    place::{place_content::TailwindPlaceContent, place_item::TailwindPlaceItems, place_self::TailwindPlaceSelf},
    shrink::TailWindShrink,
    span::{TailwindColumn, TailwindRow},
};
use crate::{
    css_attributes, syntax_error, CssAttribute, CssBehavior, LengthUnit, Result, TailwindArbitrary, TailwindBuilder,
    TailwindInstance,
};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter, Write},
};
use tailwind_ast::parse_integer;

mod basis;
mod display;
mod flex;
mod gap;
mod grid;
mod grow;
mod justify;
mod order;
mod parser;
mod place;
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
