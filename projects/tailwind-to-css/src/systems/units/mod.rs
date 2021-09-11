use crate::{syntax_error, CssBehavior, Result, TailwindArbitrary};
use nom::{
    branch::alt,
    complete::tag,
    error::{Error, ErrorKind},
    sequence::tuple,
    Err::Failure,
    IResult,
};
use std::fmt::{Display, Formatter};
use tailwind_ast::parse_f32;

pub use self::{anchor::AnchorPoint, length::LengthUnit};

mod anchor;
mod length;
