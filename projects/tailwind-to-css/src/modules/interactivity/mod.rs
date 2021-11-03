use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use crate::{css_attributes, CssAttribute, Result, TailwindArbitrary, TailwindBuilder, TailwindColor, TailwindInstance};

pub use self::{
    accent::TailwindAccentColor, appearance::TailwindAppearance, caret::TailwindCaretColor, cursor::TailwindCursor,
    pointer::TailwindPointerEvents, resize::TailwindResize, scroll::TailwindScroll, select::TailwindSelect, snap::TailwindSnap,
    torch::TailwindTorch, will_change::TailwindWillChange,
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
