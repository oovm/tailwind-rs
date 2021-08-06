use super::*;

impl TailwindDuration {
    /// https://tailwindcss.com/docs/transition-duration
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary in duration");
        match input {
            [n] => Ok(Self { ms: parse_usize(n)? }),
            _ => syntax_error!("Unknown duration instructions: {}", input.join("-")),
        }
    }
}

impl TailwindDelay {
    /// https://tailwindcss.com/docs/transition-delay
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary in duration");
        match input {
            [n] => Ok(Self { ms: parse_usize(n)? }),
            _ => syntax_error!("Unknown delay instructions: {}", input.join("-")),
        }
    }
}

#[inline(always)]
fn parse_usize(scale: &str) -> Result<usize> {
    Ok(parse_integer(scale)?.1)
}
