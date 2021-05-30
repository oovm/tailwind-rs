// #![feature(type_alias_impl_trait)]
// #![feature(box_syntax)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_crate_level_docs)]
#![doc = include_str ! ("../readme.md")]

extern crate core;

mod builder;
mod macros;
mod systems;
mod traits;

pub use self::{
    builder::{parser::*, TailwindBuilder},
    systems::{
        accessibility::TailwindScreenReader,
        breakpoints::{BreakPoint, BreakPointSystem},
        colors::{Palette, PaletteSystem},
        effects::{ShadowSystem, TailwindBlend, TailwindBlendKind, TailwindBlendMode, TailwindOpacity},
        fonts::FontSystem,
        layouts::{TailWindZIndex, TailwindAspect, TailwindBreak},
        preflight::PreflightSystem,
        sizes::{TailwindHeight, TailwindSizing, TailwindWidth},
        spaces::{TailwindSpacing, TailwindSpacingKind, TailwindSpacingSize},
        tables::{TailwindBorderCollapse, TailwindTableLayout},
    },
    traits::{CssAttribute, ParsedItem, ParsedList, TailwindInstance, TailwindObject},
};
pub use tailwind_error::{Result, TailwindError, TailwindErrorKind};
