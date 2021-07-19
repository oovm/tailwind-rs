use super::*;
mod display;
mod parser;
#[cfg(test)]
mod test;

#[derive(Copy, Clone, Debug)]
enum AspectKind {
    Auto,
    Radio(usize, usize),
    Global(CssBehavior),
}

#[doc = include_str!("aspect-ratio.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindAspect {
    kind: AspectKind,
}

#[doc = include_str!("container.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindContainer {}

#[derive(Copy, Clone, Debug)]
enum ColumnKind {
    Auto,
    Columns(u8),
    Length(LengthUnit<usize>),
    // Global(CssBehavior),
}

#[doc = include_str!("columns.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindColumns {
    kind: ColumnKind,
}

#[derive(Copy, Clone, Debug)]
enum BreakKind {
    Before,
    After,
    Inside,
}

#[doc = include_str!("break.md")]
#[derive(Clone, Debug)]
pub struct TailwindBreakLayout {
    kind: BreakKind,
    info: String,
}

/// https://tailwindcss.com/docs/box-sizing
#[derive(Copy, Clone, Debug)]
pub enum TailwindBoxDecoration {
    Clone,
    Slice,
}

/// https://tailwindcss.com/docs/box-sizing
#[derive(Copy, Clone, Debug)]
pub enum TailwindBoxSizing {
    Border,
    Content,
}

/// https://tailwindcss.com/docs/display
#[derive(Copy, Clone, Debug)]
pub enum TailwindDisplay {
    Block,
    Inline,
    InlineBlock,
    Flex,
    InlineFlex,
    Table,
    InlineTable,
    TableCaption,
}

/// https://tailwindcss.com/docs/float
#[derive(Copy, Clone, Debug)]
pub enum TailwindFloat {
    Left,
    Right,
    None,
}

/// https://tailwindcss.com/docs/clear
#[derive(Copy, Clone, Debug)]
pub enum TailwindClear {
    Left,
    Right,
    Both,
    None,
}

#[doc = include_str ! ("z-index.md")]
#[derive(Copy, Clone, Debug)]
pub enum TailWindZIndex {
    Auto,
    Positive(usize),
    Negative(usize),
}

#[doc = include_str ! ("box-sizing.md")]
#[derive(Copy, Clone, Debug)]
pub enum TailWindBoxSizing {
    Auto,
    Positive(isize),
    Negative(isize),
}

#[derive(Copy, Clone, Debug)]
pub enum TailwindIsolation {
    Isolate,
    Auto,
}

/// https://tailwindcss.com/docs/object-fit
#[derive(Copy, Clone, Debug)]
pub enum TailwindObjectFit {
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}

/// https://tailwindcss.com/docs/object-fit
#[derive(Clone, Debug)]
pub enum TailwindObjectPosition {
    LeftTop,
    Top,
    RightTop,
    Left,
    Center,
    Right,
    LeftBottom,
    Bottom,
    RightBottom,
    Custom { x: String, y: String },
}
#[derive(Copy, Clone, Debug)]
enum OverflowKind {
    Auto,
    Hidden,
    Clip,
    Visible,
    Scroll,
}

/// https://tailwindcss.com/docs/overflow#hiding-content-that-overflows
#[derive(Copy, Clone, Debug)]
pub struct TailwindOverflow {
    kind: OverflowKind,
    axis: Option<bool>,
}

#[derive(Copy, Clone, Debug)]
enum OverscrollKind {
    Auto,
    Contain,
    None,
}

/// https://tailwindcss.com/docs/overscroll-behavior
#[derive(Copy, Clone, Debug)]
pub struct TailwindOverscroll {
    kind: OverscrollKind,
    axis: Option<bool>,
}

#[doc = include_str ! ("position.md")]
#[derive(Copy, Clone, Debug)]
pub enum TailwindPosition {
    Static,
    Fixed,
    Absolute,
    Relative,
    Sticky,
}

/// https://tailwindcss.com/docs/visibility
#[derive(Copy, Clone, Debug)]
pub enum TailwindVisibility {
    Visible,
    Invisible,
}
