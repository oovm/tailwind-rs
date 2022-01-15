use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindContrast {
    percent: NumericValue,
    backdrop: Backdrop,
}

impl Display for TailwindContrast {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "contrast-{}", self.percent)
    }
}

impl TailwindInstance for TailwindContrast {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let n = self.percent.get_properties(|f| format!("{}%", f));
        self.backdrop.get_filter(format!("contrast({})", n))
    }
}

impl TailwindContrast {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = match rest {
            [] if arbitrary.is_none() => 100u32.into(),
            _ => NumericValue::positive_parser("contrast", |_| false)(rest, arbitrary)?,
        };
        Ok(Self { percent, backdrop: Backdrop::from(backdrop) })
    }
}
