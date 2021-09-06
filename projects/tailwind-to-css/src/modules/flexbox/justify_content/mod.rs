use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindJustifyContent {
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}
impl Display for TailwindJustifyContent {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindJustifyContent {}

impl TailwindJustifyContent {
    pub fn parse(_pattern: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
