use std::fmt::{Display, Formatter};

use nom::{branch::alt, bytes::complete::tag, sequence::tuple, IResult};
use tailwind_error::TailwindError;

use tailwind_ast::parse_f32;

use crate::{syntax_error, CssAttributes, CssBehavior, Result, TailwindArbitrary};

pub use self::{
    anchor::AnchorPoint, axis::SpacingAxis, axis_xy::AxisXY, integer_only::IntegerOnly, keyword_only::KeywordOnly,
    length::LengthUnit, negative::Negative,
};

mod anchor;
mod axis;
mod axis_xy;
mod integer_only;
mod keyword_only;
mod length;
mod negative;
