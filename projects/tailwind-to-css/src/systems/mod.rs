pub mod breakpoints;
pub mod builder;
pub mod colors;
pub mod css_global;
pub mod fonts;
pub mod instruction;
pub mod length;
pub mod preflight;

use crate::*;
use css_color_parser::Color;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::{Debug, Display, Formatter, Write},
    str::FromStr,
};
use tailwind_ast::*;
use tailwind_error::nom::{
    branch::alt,
    bytes::complete::tag,
    error::{Error, ErrorKind},
    sequence::tuple,
    Err::Failure,
    IResult,
};
