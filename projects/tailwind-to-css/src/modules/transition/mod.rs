mod animate;
mod delay;
mod duration;
mod ease;
mod transit;
pub use self::{
    animate::TailwindAnimate, delay::TailwindDelay, duration::TailwindDuration, ease::TailwindEase, transit::TailwindTransition,
};
use crate::{
    css_attributes, syntax_error, CssAttributes, NumericValue, Result, TailwindArbitrary, TailwindBuilder, TailwindInstance,
};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};
