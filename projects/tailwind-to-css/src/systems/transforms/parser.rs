use super::*;

impl TailwindScale {
    // https://tailwindcss.com/docs/scale
    pub fn parse(input: &[&str], arbitrary: &str, axis: Option<bool>) -> Result<Self> {
        // forbidden arbitrary
        debug_assert_eq!(arbitrary.is_empty());
        match input {
            [n] => Ok(Self { scale: Self::parse_scale(n)?, axis }),
            _ => syntax_error!("Unknown scale instructions: {}", input.join("-")),
        }
    }
    fn parse_scale(scale: &str) -> Result<usize> {
        Ok(parse_integer(scale)?.1)
    }
}
