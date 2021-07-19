pub mod breakpoints;
pub mod colors;
pub mod css_global;
pub mod fonts;
pub mod length;
pub mod preflight;

use crate::{
    parse_integer, Result, TailwindBackgroundColor, TailwindBuilder, TailwindInstance, TailwindRingColor, TailwindTextColor,
};
use css_color_parser::Color;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::{Display, Formatter, Write},
    str::FromStr,
};
