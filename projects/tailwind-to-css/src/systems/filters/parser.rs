use super::*;

// Class
// Properties
// blur-none	filter: blur(0);
// blur-sm	filter: blur(4px);
// blur	filter: blur(8px);
// blur-md	filter: blur(12px);
// blur-lg	filter: blur(16px);
// blur-xl	filter: blur(24px);
// blur-2xl	filter: blur(40px);
// blur-3xl	filter: blur(64px);

impl TailwindBlur {
    #[inline]
    pub fn parse(rest: &[&str], arbitrary: &str) -> Result<Self> {
        match rest {
            ["none"] => Self { px: 0, backdrop: false },
            [n] => Ok(Self { percent: parse_integer::<usize>(n)?.1 as f32 / 100.0, backdrop: 0 }),
            _ => syntax_error!("Unknown Brightness instructions"),
        }
    }
    #[inline]
    fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        Ok(Self { percent: parse_i_px_maybe(arbitrary)?.1 })
    }
}

impl TailwindBrightness {
    #[inline]
    pub fn parse(rest: &[&str], arbitrary: &str) -> Result<Self> {
        match rest {
            [] => Self::parse_arbitrary(arbitrary),
            [n] => Ok(Self { percent: parse_integer::<usize>(n)?.1 as f32 / 100.0, backdrop: 0 }),
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
