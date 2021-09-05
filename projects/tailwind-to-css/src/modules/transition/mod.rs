mod animate;
mod delay;
mod duration;
mod ease;
#[cfg(test)]
mod test;
mod trans;

pub use self::{
    animate::TailwindAnimate, delay::TailwindDelay, duration::TailwindDuration, ease::TailwindEase, trans::TailwindTransition,
};
use crate::{css_attributes, syntax_error, CssAttribute, Result, TailwindArbitrary, TailwindBuilder, TailwindInstance};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};
