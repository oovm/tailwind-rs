use super::*;
use nom::combinator::{map_res, recognize};
use std::str::FromStr;

/// `\d+`
pub fn parser_integer<'a, T>() -> impl FnMut(&'a str) -> IResult<&'a str, T>
where
    T: FromStr,
{
    map_res(recognize(digit1), str::parse)
}

#[test]
fn test_isize() {
    assert_eq!(parser_integer()("0"), Ok(("", 0isize)));
    assert_eq!(parser_integer()("42"), Ok(("", 42isize)));
}

#[test]
fn test_usize() {
    assert_eq!(parser_integer()("0"), Ok(("", 0usize)));
    assert_eq!(parser_integer()("42"), Ok(("", 42usize)));
}

/// `\d+\.\d+`
pub fn parser_f32<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, f32> {
    let float1 = tuple((digit1, opt(tuple((tag("."), digit1)))));
    map_res(recognize(float1), str::parse)
}

#[test]
fn test_f32() {
    assert_eq!(parser_f32()("0"), Ok(("", 0.0)));
    assert_eq!(parser_f32()("42"), Ok(("", 42.0)));
    assert_eq!(parser_f32()("99.99"), Ok(("", 99.99)));
}

/// `\d+\/\d+`
pub fn parser_fraction<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, (usize, usize)> {
    move |input| match tuple((parser_integer(), tag("/"), parser_integer()))(input) {
        Ok((rest, (a, _, b))) => Ok((rest, (a, b))),
        Err(e) => Err(e),
    }
}

#[test]
fn test_fraction() {
    assert_eq!(parser_fraction()("1/12"), Ok(("", (1, 12))));
    assert_eq!(parser_fraction()("12/2"), Ok(("", (12, 2))));
    assert_eq!(parser_fraction()("12/24"), Ok(("", (12, 24))));
}

/// #50d71e
pub fn parser_color_hex() {}
