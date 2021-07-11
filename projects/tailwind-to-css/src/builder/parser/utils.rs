use super::*;

/// `\d+`
pub fn parse_integer<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    map_res(recognize(digit1), str::parse)(input)
}

pub fn parse_i_px_maybe<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    let (rest, (i, _)) = tuple((parse_integer, opt(tag("px"))))(input)?;
    Ok((rest, i))
}

#[test]
fn test_isize() {
    assert_eq!(parse_integer("0"), Ok(("", 0isize)));
    assert_eq!(parse_integer("42"), Ok(("", 42isize)));
}

#[test]
fn test_usize() {
    assert_eq!(parse_integer("0"), Ok(("", 0usize)));
    assert_eq!(parse_integer("42"), Ok(("", 42usize)));
}

/// `\d+\.\d+`
pub fn parse_f32(input: &str) -> IResult<&str, f32> {
    let float1 = tuple((digit1, opt(tuple((tag("."), digit1)))));
    map_res(recognize(float1), str::parse)(input)
}

#[test]
fn test_f32() {
    assert_eq!(parse_f32("0"), Ok(("", 0.0)));
    assert_eq!(parse_f32("42"), Ok(("", 42.0)));
    assert_eq!(parse_f32("99.99"), Ok(("", 99.99)));
}

/// `\d+\/\d+`
pub fn parse_fraction(input: &str) -> IResult<&str, (usize, usize)> {
    let (rest, (a, _, b)) = tuple((parse_integer, tag("/"), parse_integer))(input)?;
    Ok((rest, (a, b)))
}

#[test]
fn test_fraction() {
    assert_eq!(parse_fraction("1/12"), Ok(("", (1, 12))));
    assert_eq!(parse_fraction("12/2"), Ok(("", (12, 2))));
    assert_eq!(parse_fraction("12/24"), Ok(("", (12, 24))));
}

/// #50d71e
pub fn parser_color_hex() {}
