use super::*;

pub(crate) mod stroke_color;
pub(crate) mod stroke_width;

#[derive(Clone, Debug)]
pub struct TailwindStroke {}

impl TailwindStroke {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        todo!()
    }
}
