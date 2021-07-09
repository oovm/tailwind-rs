use super::*;

impl TailwindScale {
    // https://tailwindcss.com/docs/scale
    pub fn parse(input: &[&str], arbitrary: &str, axis: Option<bool>, neg: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_empty(), "forbidden arbitrary");
        match input {
            [n] => Ok(Self { neg, scale: parse_usize(n)?, axis }),
            _ => syntax_error!("Unknown scale instructions: {}", input.join("-")),
        }
    }
}

impl TailwindRotate {
    // https://tailwindcss.com/docs/rotate
    pub fn parse(input: &[&str], arbitrary: &str, neg: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_empty(), "forbidden arbitrary");
        match input {
            [n] => Ok(Self { neg, deg: parse_usize(n)? }),
            _ => syntax_error!("Unknown rotate instructions: {}", input.join("-")),
        }
    }
}

impl TailwindSkew {
    // https://tailwindcss.com/docs/skew
    pub fn parse(input: &[&str], arbitrary: &str, axis: bool, neg: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_empty(), "forbidden arbitrary");
        match input {
            [n] => Ok(Self { neg, deg: parse_usize(n)?, axis }),
            _ => syntax_error!("Unknown rotate instructions: {}", input.join("-")),
        }
    }
}

#[inline(always)]
fn parse_usize(deg: &str) -> Result<usize> {
    Ok(parse_integer(deg)?.1)
}
