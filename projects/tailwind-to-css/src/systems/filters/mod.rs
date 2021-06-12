use super::*;
use crate::parse_f32;

pub struct TailwindBrightness {
    brightness: f32,
}

impl TailwindBrightness {
    #[inline]
    pub fn parse(rest: &[&str], arbitrary: &str) -> Result<Self> {
        match rest {
            [] => Self::parse_arbitrary(arbitrary),
            [n] => Ok(Self { brightness: parse_integer::<usize>(n)?.1 as f32 / 100.0 }),
            _ => syntax_error!("Unknown Brightness instructions"),
        }
    }
    #[inline]
    fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        Ok(Self { brightness: parse_f32(arbitrary)?.1 })
    }
}

impl Display for TailwindBrightness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "brightness-{}", (self.brightness * 100.0) as usize)
    }
}

impl TailwindInstance for TailwindBrightness {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let brightness = format!("brightness({})", self.brightness);
        css_attributes! {
            "filter" => brightness
        }
    }
}
