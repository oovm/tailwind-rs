use super::*;
use crate::parse_f32;

#[doc = include_str ! ("brightness.md")]
#[derive(Clone, Debug)]
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
#[test]
fn build_brightness() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("brightness-125"));
    assert_eq!(out, "{filter: brightness(1.25);}")
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
