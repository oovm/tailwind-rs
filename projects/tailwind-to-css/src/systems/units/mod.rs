pub use self::{anchor::AnchorPoint, length::LengthUnit};
use crate::{syntax_error, CssBehavior, Result, TailwindArbitrary};
use nom::{branch::alt, bytes::complete::tag, sequence::tuple, IResult};
use std::fmt::{Display, Formatter};
use tailwind_ast::parse_f32;

mod anchor;
mod length;
