use crate::CssAttribute;
use nom::{branch::alt, bytes::complete::tag, sequence::tuple, IResult};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use tailwind_ast::parse_f32;

use crate::{syntax_error, CssBehavior, Result, TailwindArbitrary};

pub use self::{
    anchor::AnchorPoint, axis::SpacingAxis, length::LengthUnit, maybe_arbitrary::MaybeArbitrary,
    never_arbitrary::NeverArbitrary,
};

mod anchor;
mod axis;
mod length;
mod maybe_arbitrary;
mod never_arbitrary;
