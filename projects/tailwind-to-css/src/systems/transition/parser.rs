use super::*;

impl TailwindDuration {
    // https://tailwindcss.com/docs/scale
    pub fn parse(input: &[&str], arbitrary: &str, axis: Option<bool>) -> Result<Self> {
        debug_assert!(arbitrary.is_empty(), "forbidden arbitrary in duration");
        todo!()
    }
    fn parse_scale(scale: &str) -> Result<usize> {
        Ok(parse_integer(scale)?.1)
    }
}
