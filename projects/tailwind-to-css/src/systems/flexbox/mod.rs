mod display;
mod parser;
use super::*;

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindFlexBasis {
    Auto,
    Full,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindFlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindFlexWrap {
    Wrap,
    WrapReverse,
    NoWrap,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindFlex {
    None,
    Inherit,
    Auto { grow: usize, shrink: usize },
    Percent { grow: usize, shrink: usize, basis: usize },
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailWindGrow {
    grow: usize,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailWindShrink {
    shrink: usize,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailWindOrder {
    order: isize,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindGridTemplate {
    row: bool,
    unit: usize,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindColumn {
    unit: usize,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindGridRow {
    unit: usize,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindRow {
    unit: usize,
}

#[doc=include_str!("justify-content.md")]
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

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindGridAuto {
    kind: GridAutoKind,
    layout: bool,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindGap {
    Row,
    Column,
    Both,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindJustifyContent {
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}

#[doc=include_str!("justify-items.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindJustifyItems {
    Start,
    End,
    Center,
    Stretch,
}

#[doc=include_str!("justify-self.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindJustifySelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}

#[doc=include_str!("justify-self.md")]
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

#[doc=include_str!("justify-self.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindItems {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}

#[doc=include_str!("justify-self.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindSelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}

#[doc=include_str!("justify-self.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindPlaceContent {
    // Auto,
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}

#[doc=include_str!("justify-self.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindPlaceItems {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}

#[doc=include_str!("justify-self.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindPlaceSelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}
