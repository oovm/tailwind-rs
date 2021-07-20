pub mod breakpoints;
pub mod colors;
pub mod css_global;
pub mod fonts;
pub mod length;
pub mod preflight;

use crate::{
    parse_f32, parse_integer, Result, TailwindBackgroundColor, TailwindBuilder, TailwindInstance, TailwindRingColor,
    TailwindTextColor,
};
use css_color_parser::Color;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::{Debug, Display, Formatter, Write},
    str::FromStr,
};
use tailwind_error::nom::{
    branch::alt,
    bytes::complete::tag,
    error::{Error, ErrorKind},
    sequence::tuple,
    Err::Failure,
    IResult,
};
