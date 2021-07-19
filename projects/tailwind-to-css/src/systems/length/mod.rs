use super::*;
use crate::{parse_f32, syntax_error};
use std::fmt::Debug;
use tailwind_error::nom::{branch::alt, bytes::complete::tag, sequence::tuple, IResult};

#[derive(Copy, Clone)]
pub enum LengthUnit<T> {
    Px(T),
    Em(T),
    Rem(T),
    Percent(T),
}

impl<T> Debug for LengthUnit<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Px(n) => write!(f, "{}px", n),
            Self::Em(n) => write!(f, "{}px", n),
            Self::Rem(n) => write!(f, "{}px", n),
            Self::Percent(n) => write!(f, "{}px", n),
        }
    }
}

impl<T> Display for LengthUnit<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Px(n) => write!(f, "{}px", *n as usize),
            Self::Em(n) => write!(f, "{}px", *n as usize),
            Self::Rem(n) => write!(f, "{}px", *n as usize),
            Self::Percent(n) => write!(f, "{}px", *n as usize),
        }
    }
}

impl<T> LengthUnit<T> {
    pub fn parse_f(input: &str) -> Result<Self>
    where
        T: From<f32>,
    {
        let (f, unit) = tuple((parse_f32, Self::parse_unit))(input)?.1;
        Self::map_unit(f, unit)
    }

    pub fn parse_i(input: &str) -> Result<Self>
    where
        T: From<usize>,
    {
        let (i, unit): (usize, &str) = tuple((parse_integer, Self::parse_unit))(input)?.1;
        Self::map_unit(i, unit)
    }

    fn map_unit<I>(input: I, unit: &str) -> Result<Self>
    where
        T: From<I>,
    {
        match unit {
            "px" => Ok(Self::Px(T::from(input))),
            "em" => Ok(Self::Em(T::from(input))),
            "rem" => Ok(Self::Rem(T::from(input))),
            "%" => Ok(Self::Percent(T::from(input))),
            _ => syntax_error!("Unknown unit {}", unit),
        }
    }

    fn parse_unit(input: &str) -> IResult<&str, &str> {
        alt((tag("px"), tag("em"), tag("rem"), tag("%")))(input)
    }
}
