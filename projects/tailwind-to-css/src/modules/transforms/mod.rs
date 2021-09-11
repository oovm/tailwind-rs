use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
};

use crate::{css_attributes, syntax_error, CssAttribute, Result, TailwindArbitrary, TailwindBuilder, TailwindInstance};

pub use self::{
    origin::TailwindOrigin, rotate::TailwindRotate, scale::TailwindScale, skew::TailwindSkew, translate::TailwindTranslate,
};

mod origin;
mod rotate;
mod scale;
mod skew;
mod translate;
