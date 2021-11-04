use super::*;

pub(crate) mod snap_align;
pub(crate) mod snap_stop;
pub(crate) mod snap_type;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindSnap {}

impl TailwindSnap {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        match pattern {
            ["start"] => Ok(TailwindSnapStop::from("start").boxed()),
            _ => syntax_error!(""),
        }
    }
    pub fn check_valid(mode: &str) -> bool {
        todo!()
    }
}
