use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
};

use crate::{css_attributes, syntax_error, CssAttribute, Result, TailwindArbitrary, TailwindBuilder, TailwindInstance};

pub use self::{
    origin::TailwindOrigin, rotate::TailwindRotate, scale::TailwindScale, skew::TailwindSkew, translate::TailwindTranslate,
};
use crate::{AnchorPoint, CssBehavior, LengthUnit};
use std::fmt::Write;

mod origin;
mod rotate;
mod scale;
mod skew;
mod translate;
