// #![feature(type_alias_impl_trait)]
// #![feature(box_syntax)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_crate_level_docs)]
#![doc = include_str ! ("../readme.md")]

mod builder;
mod macros;
mod systems;
mod traits;

pub use self::{
    builder::TailwindBuilder,
    systems::{
        breakpoints::{BreakPoint, BreakPointSystem},
        colors::{Palette, PaletteSystem},
        effects::{ShadowSystem, TailwindBlendKind, TailwindBlendMode, TailwindBlender, TailwindOpacity},
        fonts::FontSystem,
        layouts::{TailWindZIndex, TailwindAspect, TailwindBreak},
        preflight::PreflightSystem,
        sizes::{TailwindHeight, TailwindSizing, TailwindWidth},
        tables::{TailwindBorderCollapse, TailwindTableLayout},
    },
    traits::{CssAttribute, TailwindInstance, TailwindObject, ParsedItem, ParsedList},
};
pub use tailwind_error::{Result, TailwindError, TailwindErrorKind};
