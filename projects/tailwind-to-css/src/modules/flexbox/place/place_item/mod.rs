use crate::modules::flexbox::*;
#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindPlaceItems {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}
impl Display for TailwindPlaceItems {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindPlaceItems {}

impl TailwindPlaceItems {
    pub fn parse(_pattern: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
