use std::str::FromStr;
#[cfg(test)]
mod tests;

/// `\d+`
pub fn parse_integer<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    map_res(recognize(digit1), str::parse)(input)
}
///
pub fn parse_i_px_maybe<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    let (rest, (i, _)) = tuple((parse_integer, opt(tag("px"))))(input)?;
    Ok((rest, i))
}

/// `\d+\.\d+`
pub fn parse_f32(input: &str) -> IResult<&str, f32> {
    let float1 = tuple((digit1, opt(tuple((tag("."), digit1)))));
    map_res(recognize(float1), str::parse)(input)
}
///
pub fn parse_f_percent(input: &str) -> IResult<&str, f32> {
    let (rest, (f, _)) = tuple((parse_f32, char('%')))(input)?;
    Ok((rest, f))
}

/// `\d+\/\d+`
pub fn parse_fraction(input: &str) -> IResult<&str, (usize, usize)> {
    let (rest, (a, _, b)) = tuple((parse_integer, tag("/"), parse_integer))(input)?;
    Ok((rest, (a, b)))
}

/// 100(/50)?
#[inline]
pub fn parse_fraction_maybe(input: &str) -> IResult<&str, (usize, Option<usize>)> {
    let (rest, (a, b)) = tuple((parse_integer, opt(tuple((tag("/"), parse_integer)))))(input)?;
    Ok((rest, (a, b.map(|s| s.1))))
}

/// #50d71e
pub fn parser_color_hex() {}

// fn hex3() {
//
// }
//
// fn hex6() {
//
// }
//
// fn hex8() {
//
// }
