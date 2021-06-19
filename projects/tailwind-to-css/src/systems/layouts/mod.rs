use super::*;

pub mod builder;
pub mod display;

#[doc = include_str!("aspect-ratio.md")]
#[derive(Copy, Clone, Debug)]
pub enum TailwindAspect {
    Auto,
    Arbitrary(usize, usize),
}

#[doc = include_str!("container.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindContainer {}

#[doc = include_str!("columns.md")]
#[derive(Copy, Clone, Debug)]
pub enum TailwindColumns {
    Auto,
    Columns(u8),
    Rem(usize),
}

#[derive(Copy, Clone, Debug)]
enum TailwindBreakKind {
    Before,
    After,
    Inside,
}

#[doc = include_str!("break.md")]
#[derive(Clone, Debug)]
pub struct TailwindBreak {
    kind: TailwindBreakKind,
    info: String,
}

/// https://tailwindcss.com/docs/box-sizing
#[derive(Copy, Clone, Debug)]
pub enum TailwindBoxDecorationBreak {
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
pub enum TailwindObjectFit {}

/// https://tailwindcss.com/docs/overscroll-behavior
#[derive(Copy, Clone, Debug)]
pub enum TailwindOverscroll {}

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
