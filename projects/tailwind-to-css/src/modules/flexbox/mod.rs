pub use self::{
    basis::TailwindBasis,
    content::{content_align::TailwindContentAlign, TailwindContent},
    flex::{flex_adaptor, flex_direction::TailwindFlexDirection, flex_wrap::TailwindFlexWrap, TailwindFlex},
    gap::TailwindGap,
    grid::{
        grid_auto::TailwindGridAuto, grid_cols::TailwindGridColumns, grid_flow::TailwindGridFlow, grid_rows::TailwindGridRows,
        TailwindGrid,
    },
    grow::TailWindGrow,
    items::TailwindItems,
    justify::{justify_content::TailwindJustifyContent, justify_item::TailwindJustifyItems, justify_self::TailwindJustifySelf},
    order::TailWindOrder,
    place::{
        place_content::TailwindPlaceContent, place_item::TailwindPlaceItems, place_self::TailwindPlaceSelf, TailwindPlace,
    },
    shrink::TailWindShrink,
    span::{TailwindColumn, TailwindRow},
    zelf::TailwindSelf,
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
mod content;
mod flex;
mod gap;
mod grid;
mod grow;
mod items;
mod justify;
mod order;
mod place;
mod shrink;
mod span;
#[cfg(test)]
mod test;
mod zelf;
