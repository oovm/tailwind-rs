use super::*;
#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindPlaceSelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}
impl Display for TailwindPlaceSelf {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindPlaceSelf {}

impl TailwindPlaceSelf {
    pub fn parse(_pattern: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
