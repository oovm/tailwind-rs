use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter, Write},
};

use crate::{
    css_attributes, syntax_error, AnchorPoint, CssAttribute, CssBehavior, LengthUnit, Result, TailwindArbitrary,
    TailwindBuilder, TailwindInstance,
};

pub use self::{
    aspect_ratio::TailwindAspect,
    boxing::{box_decoration::TailwindBoxDecoration, box_sizing::TailwindBoxSizing},
    breaking::{after::TailwindBreakAfter, before::TailwindBreakBefore, inside::TailwindBreakInside},
    clear::TailwindClear,
    columns::TailwindColumns,
    container::TailwindContainer,
    display::TailwindDisplay,
    float::TailwindFloat,
    isolate::TailwindIsolation,
    object::*,
    overflow::TailwindOverflow,
    overscroll::TailwindOverscroll,
    placement::{bottom::TailwindBottom, inset::TailwindInset, left::TailwindLeft, right::TailwindRight, top::TailwindTop},
    position::TailwindPosition,
    visible::TailwindVisibility,
    z_index::TailwindZIndex,
};

mod aspect_ratio;
mod boxing;
mod breaking;
mod clear;
mod columns;
mod container;
mod display;
mod float;
mod isolate;
mod object;
mod overflow;
mod overscroll;
mod placement;
mod position;
#[cfg(test)]
mod test;
mod visible;
mod z_index;
