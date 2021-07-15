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
pub mod theme;
pub mod transforms;
pub mod transition;
pub mod typography;

use crate::{
    css_attributes, parse_f32, parse_f_percent, parse_i_px_maybe, parse_integer, syntax_error, traits::CssAttribute,
    ColorResolver, TailwindBrightness, TailwindBuilder, TailwindInstance, TailwindObjectPosition,
};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::{Debug, Display, Formatter, Write},
    str::FromStr,
};
use tailwind_error::{
    nom::{branch::alt, bytes::complete::tag, combinator::opt, sequence::tuple, IResult},
    Result,
};

///
#[derive(Debug, Copy, Clone)]
pub enum CssBehavior {
    Inherit,
    Initial,
    Unset,
}
