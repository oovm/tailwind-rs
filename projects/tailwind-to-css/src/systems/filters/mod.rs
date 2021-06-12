use super::*;
use crate::parse_f32;

pub struct TailwindBrightness {
    brightness: f32,
}

impl TailwindBrightness {
    #[inline]
    pub fn parse(kind: &[&str], arbitrary: &str) -> Result<Self> {
        match kind {
            [] => Self::parse_arbitrary(arbitrary),
            [n] => Ok(Self { brightness: parse_integer::<usize>(n)?.1 as f32 / 100.0 }),
            _ => {
                syntax_error!("Unknown Brightness instructions")
            }
        }
    }
    #[inline]
    fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        Ok(Self { brightness: parse_f32(arbitrary)?.1 })
    }
}
