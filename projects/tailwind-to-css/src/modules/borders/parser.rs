use super::*;
use tailwind_ast::parse_i_px_maybe;

impl TailwindBorderStyle {
    pub fn into_instance(self) -> Box<dyn TailwindInstance> {
        Box::new(self)
    }
}

impl TailwindRingOffsetWidth {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match input {
            [] => Self::parse_arbitrary(arbitrary),
            [n] => Self::parse_arbitrary(&TailwindArbitrary::from(*n)),
            _ => syntax_error!("unknown aspect-ratio elements"),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { width: parse_i_px_maybe(arbitrary.as_str())?.1 })
    }
}
