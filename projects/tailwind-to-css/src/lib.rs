// #![feature(type_alias_impl_trait)]
// #![feature(box_syntax)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_crate_level_docs)]
#![doc = include_str ! ("../readme.md")]

mod builder;
mod systems;
mod traits;
mod macros;
pub use self::{
    builder::TailwindBuilder,
    systems::{
        breakpoints::{BreakPoint, BreakPointSystem},
        colors::{Palette, PaletteSystem},
        fonts::FontSystem,
        layouts::{TailWindZIndex, TailwindAspect, TailwindBreak},
        preflight::PreflightSystem,
        sizes::{TailwindSizing, TailwindWidth},
        TailwindObject, TailwindParsed,
    },
    traits::{TailwindInstance,CssAttribute},
};
pub use tailwind_error::{Result, TailwindError, TailwindErrorKind};
