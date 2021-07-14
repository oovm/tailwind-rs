// #![feature(type_alias_impl_trait)]
// #![feature(box_syntax)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/67109815")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/67109815")]

extern crate core;

mod builder;
mod macros;
mod systems;
mod traits;

pub use self::{
    builder::*,
    systems::{
        accessibility::*, background::*, borders::*, breakpoints::*, colors::*, effects::*, filters::*, flexbox::*, fonts::*,
        interactivity::*, layouts::*, preflight::*, sizing::*, spacing::*, tables::*, transforms::*, transition::*,
        typography::*,
    },
    traits::{CssAttribute, ParsedItem, ParsedList, TailwindInstance, TailwindObject},
};
pub use tailwind_error::{Result, TailwindError, TailwindErrorKind};
