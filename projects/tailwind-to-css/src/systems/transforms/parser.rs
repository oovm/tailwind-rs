use super::*;

impl TailwindScale {
    // https://tailwindcss.com/docs/scale
    pub fn parse(input: &[&str], arbitrary: &str, axis: Option<bool>) -> Result<Self> {
        debug_assert!(arbitrary.is_empty(), "forbidden arbitrary");
        match input {
            [n] => Ok(Self { scale: Self::parse_scale(n)?, axis }),
            _ => syntax_error!("Unknown scale instructions: {}", input.join("-")),
        }
    }
    fn parse_scale(scale: &str) -> Result<usize> {
        Ok(parse_integer(scale)?.1)
    }
}

impl TailwindRotate {
    // https://tailwindcss.com/docs/rotate
    pub fn parse(input: &[&str], arbitrary: &str) -> Result<Self> {
        debug_assert!(arbitrary.is_empty(), "forbidden arbitrary");
        match input {
            [n] => Ok(Self { deg: parse_deg(n)? }),
            _ => syntax_error!("Unknown rotate instructions: {}", input.join("-")),
        }
    }
    #[inline]
    fn parse_deg(deg: &str) -> Result<usize> {
        Ok(parse_integer(deg)?.1)
    }
}

impl TailwindSkew {
    // https://tailwindcss.com/docs/skew
    pub fn parse(input: &[&str], arbitrary: &str, axis: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_empty(), "forbidden arbitrary");
        match input {
            [n] => Ok(Self { deg: parse_deg(n)?, axis }),
            _ => syntax_error!("Unknown rotate instructions: {}", input.join("-")),
        }
    }
}

#[inline]
fn parse_deg(deg: &str) -> Result<usize> {
    Ok(parse_integer(deg)?.1)
}
