use super::*;

mod snap_align;
mod snap_stop;
mod snap_strictness;
mod snap_type;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindSnap {}

impl Display for TailwindSnap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindSnap {}

impl TailwindSnap {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        todo!()
    }
    pub fn check_valid(mode: &str) -> bool {
        todo!()
    }
}
