pub mod breakpoints;
pub mod builder;
pub mod colors;
pub mod css_global;
pub mod fonts;
pub mod instruction;
pub mod length;
pub mod preflight;
pub mod processor;

use crate::*;
use parcel_css::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display, Formatter, Write},
    str::FromStr,
};
use tailwind_ast::*;
use tailwind_error::{
    nom::{
        branch::alt,
        bytes::complete::tag,
        error::{Error, ErrorKind},
        sequence::tuple,
        Err::Failure,
        IResult,
    },
    Result,
};
