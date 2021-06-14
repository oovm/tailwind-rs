use super::*;

impl TailwindBackgroundBrightness {
    #[inline]
    pub fn parse(rest: &[&str], arbitrary: &str) -> Result<Self> {
        Ok(Self { brightness: TailwindBrightness::parse(rest, arbitrary)? })
    }
}
