mod display;
mod parser;
use super::*;

#[derive(Debug, Copy, Clone)]
enum BasisSize {}

#[doc=include_str!("flex-basis.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindBasis {
    size: BasisSize,
}

#[doc=include_str!("flex-direction.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindFlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[doc=include_str!("flex-wrap.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindFlexWrap {
    Wrap,
    WrapReverse,
    NoWrap,
}

#[doc=include_str!("flex.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindFlex {
    None,
    Inherit,
    Auto { grow: usize, shrink: usize },
    Percent { grow: usize, shrink: usize, basis: usize },
}

#[doc=include_str!("flex-grow.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailWindGrow {
    grow: usize,
}

#[doc=include_str!("flex-shrink.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailWindShrink {
    shrink: usize,
}

#[doc=include_str!("order.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailWindOrder {
    order: isize,
}

#[doc=include_str!("grid-template-columns.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindGridTemplate {
    row: bool,
    unit: usize,
}

#[derive(Debug, Copy, Clone)]
enum SpanKind {}

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindRow {
    span: SpanKind,
}

#[doc=include_str!("grid-column.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindColumn {
    span: SpanKind,
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

#[doc=include_str!("place-content.md")]
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

#[doc=include_str!("place-items.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindPlaceItems {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}

#[doc=include_str!("place-self.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindPlaceSelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}
