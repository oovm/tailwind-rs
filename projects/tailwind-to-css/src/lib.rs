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
        accessibility::*, background::*, borders::*, breakpoints::*, colors::*, effects::*, filters::*, fonts::*, layouts::*,
        preflight::*, sizes::*, spaces::*, tables::*, typography::*,
    },
    traits::{CssAttribute, ParsedItem, ParsedList, TailwindInstance, TailwindObject},
};
pub use tailwind_error::{Result, TailwindError, TailwindErrorKind};
