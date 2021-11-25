use crate::CssAttribute;
use nom::{branch::alt, bytes::complete::tag, sequence::tuple, IResult};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use tailwind_ast::parse_f32;

use crate::{syntax_error, CssBehavior, Result, TailwindArbitrary};

pub use self::{anchor::AnchorPoint, axis::SpacingAxis, length::LengthUnit, standard::KeywordOnly};

mod anchor;
mod axis;
mod length;
mod standard;
