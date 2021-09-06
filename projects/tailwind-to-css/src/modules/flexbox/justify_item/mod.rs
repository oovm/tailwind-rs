use super::*;
#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindJustifyItems {
    Start,
    End,
    Center,
    Stretch,
}
impl Display for TailwindJustifyItems {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindJustifyItems {}

impl TailwindJustifyItems {
    pub fn parse(_pattern: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
