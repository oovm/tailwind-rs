use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindGrayscale {
    percent: NumericValue,
    backdrop: Backdrop,
}
impl Display for TailwindGrayscale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "grayscale-{}", self.percent)
    }
}

impl TailwindInstance for TailwindGrayscale {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let n = self.percent.get_properties(|f| format!("{}%", f));
        self.backdrop.get_filter(format!("grayscale({})", n))
    }
}

impl TailwindGrayscale {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = match rest {
            [] if arbitrary.is_none() => 100u32.into(),
            _ => NumericValue::positive_parser("grayscale", |_| false)(rest, arbitrary)?,
        };
        Ok(Self { percent, backdrop: Backdrop::from(backdrop) })
    }
}
