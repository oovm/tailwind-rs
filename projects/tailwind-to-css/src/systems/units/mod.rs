use std::fmt::{Display, Formatter};

use crate::{TailwindBuilder, TailwindInstance};
use nom::{branch::alt, bytes::complete::tag, sequence::tuple, IResult};
use tailwind_ast::parse_f32;
use tailwind_error::TailwindError;

pub use self::{
    anchor::AnchorPoint,
    axis::SpacingAxis,
    axis_xy::AxisXY,
    integer_only::NumericValue,
    keyword_only::{KeywordInstance, KeywordMap, StandardValue},
    length::LengthUnit,
    length_only::UnitValue,
    negative::Negative,
};
use crate::{syntax_error, CssAttributes, Result, TailwindArbitrary};

mod anchor;
mod axis;
mod axis_xy;
mod integer_only;
mod keyword_only;
mod length;
mod length_only;
mod negative;
