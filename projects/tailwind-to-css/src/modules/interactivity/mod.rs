pub use self::{
    accent::TailwindAccentColor, caret::TailwindCaretColor, cursor::TailwindCursor, pointer::TailwindPointerEvents,
    resize::TailwindResize, select::TailwindSelect, torch::TailwindTorch, will_change::TailwindWillChange,
};
use crate::{
    css_attributes, syntax_error, CssAttribute, CssBehavior, LengthUnit, Result, TailwindArbitrary, TailwindBuilder,
    TailwindColor, TailwindInstance,
};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter, Write},
};

mod accent;
mod appearance;
mod caret;
mod cursor;
mod pointer;
mod resize;
mod scroll;
mod select;
mod snap;
#[cfg(test)]
mod test;
mod torch;
mod will_change;
