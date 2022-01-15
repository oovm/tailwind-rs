use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBrightness {
    percent: NumericValue,
    backdrop: Backdrop,
}

impl Display for TailwindBrightness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "brightness-{}", self.percent)
    }
}

impl TailwindInstance for TailwindBrightness {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let n = self.percent.get_properties(|f| format!("{}%", f));
        self.backdrop.get_filter(format!("brightness({})", n))
    }
}

impl TailwindBrightness {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = match rest {
            [] if arbitrary.is_none() => 100u32.into(),
            _ => NumericValue::positive_parser("brightness", |_| false)(rest, arbitrary)?,
        };
        Ok(Self { percent, backdrop: Backdrop::from(backdrop) })
    }
}
