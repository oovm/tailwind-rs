#![allow(non_upper_case_globals)]
pub mod accessibility;
pub mod background;
pub mod borders;
pub mod effects;
pub mod filters;
pub mod flexbox;
pub mod interactivity;
pub mod layouts;
pub mod sizing;
pub mod spacing;
pub mod tables;
pub mod transforms;
pub mod transition;
pub mod typography;

use crate::*;
use std::{
    collections::{BTreeSet, HashMap},
    fmt::{Debug, Display, Formatter, Write},
    str::FromStr,
};
use tailwind_ast::{parse_f32, parse_f_percent, parse_fraction, parse_i_px_maybe, parse_integer};
use tailwind_error::{
    nom::{branch::alt, bytes::complete::tag, sequence::tuple, IResult},
    Result,
};
