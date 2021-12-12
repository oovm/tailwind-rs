use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use nom::{branch::alt, bytes::complete::tag, sequence::tuple, IResult};
use tailwind_error::TailwindError;

use tailwind_ast::parse_f32;

use crate::{syntax_error, CssAttribute, CssBehavior, Result, TailwindArbitrary};

pub use self::{
    anchor::AnchorPoint, axis::SpacingAxis, integer_only::IntegerOnly, keyword_only::KeywordOnly, length::LengthUnit,
};

mod anchor;
mod axis;
mod integer_only;
mod keyword_only;
mod length;
