use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use crate::{
    css_attributes, CssAttributes, Negative, Result, StandardValue, TailwindArbitrary, TailwindBuilder, TailwindColor,
    TailwindInstance, TailwindScrollMargin, TailwindScrollPadding,
};

pub use self::{
    accent::TailwindAccentColor,
    appearance::TailwindAppearance,
    caret::TailwindCaretColor,
    cursor::TailwindCursor,
    pointer::TailwindPointerEvents,
    resize::TailwindResize,
    scroll::scroll_behavior::TailwindScrollBehavior,
    select::TailwindSelect,
    snap::{snap_align::TailwindSnapAlign, snap_stop::TailwindSnapStop, snap_type::TailwindSnapType},
    torch::TailwindTorch,
    will_change::TailwindWillChange,
};
pub(crate) use self::{scroll::scroll_adaptor, snap::snap_adaptor};

mod accent;
mod appearance;
mod caret;
mod cursor;
mod pointer;
mod resize;
mod scroll;
mod select;
mod snap;
mod torch;
mod will_change;
