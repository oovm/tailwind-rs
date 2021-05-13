use nom::combinator::{map_res, recognize};
use super::*;


pub fn parser_isize<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, isize> {
    map_res(recognize(digit1), str::parse)
}

pub fn parser_usize<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, usize> {
    map_res(recognize(digit1), str::parse)
}

pub fn parser_f32<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, f32> {
    let float1 = tuple((digit1, opt(tuple((tag("."), digit1)))));
    map_res(recognize(float1), str::parse)
}


pub fn parser_fraction() {}

#[test]
fn test_isize() {
    assert_eq!(parser_isize()("0"), Ok(("", 0isize)));
    assert_eq!(parser_isize()("42"), Ok(("", 42isize)));
}

#[test]
fn test_usize() {
    assert_eq!(parser_usize()("0"), Ok(("", 0usize)));
    assert_eq!(parser_usize()("42"), Ok(("", 42usize)));
}


#[test]
fn test_f32() {
    assert_eq!(parser_f32()("0"), Ok(("", 0.0)));
    assert_eq!(parser_f32()("42"), Ok(("", 42.0)));
    assert_eq!(parser_f32()("99.99"), Ok(("", 99.99)));
}