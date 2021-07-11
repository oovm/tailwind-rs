use super::*;

impl TailwindBrightness {
    #[inline]
    pub fn parse(rest: &[&str], arbitrary: &str) -> Result<Self> {
        match rest {
            [] => Self::parse_arbitrary(arbitrary),
            [n] => Ok(Self { percent: parse_integer::<usize>(n)?.1 as f32 / 100.0 }),
            _ => syntax_error!("Unknown Brightness instructions"),
        }
    }
    #[inline]
    fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        Ok(Self { percent: parse_f32(arbitrary)?.1 })
    }
}
#[test]
fn build_brightness() {
    let builder = TailwindBuilder::default();
    let out = format!("{:?}", builder.inline("brightness-125"));
    assert_eq!(out, "{filter: brightness(1.25);}")
}
