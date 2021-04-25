// #![feature(type_alias_impl_trait)]
// #![feature(box_syntax)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_crate_level_docs)]
#![doc = include_str ! ("../readme.md")]

extern crate core;

mod builder;
mod systems;
mod traits;

pub use self::{
    builder::TailwindBuilder,
    systems::{
        breakpoints::{BreakPoint, BreakPointSystem},
        colors::{Palette, PaletteSystem},
        fonts::FontSystem,
        layouts::{TailwindAspect, TailwindBreak},
        preflight::PreflightSystem,
        sizes::{TailwindMaxWidth, TailwindMinWidth, TailwindWidth},
        TailwindObject,
    },
    traits::TailwindInstance,
};
pub use tailwind_error::{Result, TailwindError, TailwindErrorKind};
