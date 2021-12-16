#![allow(non_upper_case_globals)]
#![allow(clippy::enum_variant_names)]
mod accessibility;
mod background;
mod borders;
mod effects;
mod filters;
mod flexbox;
mod interactivity;
mod layouts;
mod sizing;
mod spacing;
mod svg;
mod tables;
mod transforms;
mod transition;
mod typography;

pub use self::{
    accessibility::*, background::*, borders::*, effects::*, filters::*, flexbox::*, interactivity::*, layouts::*, sizing::*,
    spacing::*, svg::*, tables::*, transforms::*, transition::*, typography::*,
};

use crate::*;
use std::fmt::{Debug, Display, Formatter};
use tailwind_error::Result;
