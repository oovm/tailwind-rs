use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindSaturate {
    percent: NumericValue,
    backdrop: Backdrop,
}

impl Display for TailwindSaturate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "saturate-{}", self.percent)
    }
}

impl TailwindInstance for TailwindSaturate {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let n = self.percent.get_properties(|f| format!("{}%", f));
        self.backdrop.get_filter(format!("saturate({})", n))
    }
}

impl TailwindSaturate {
    /// <https://tailwindcss.com/docs/saturate>
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = match rest {
            [] if arbitrary.is_none() => 100u32.into(),
            _ => NumericValue::positive_parser("saturate", |_| false)(rest, arbitrary)?,
        };
        Ok(Self { percent, backdrop: Backdrop::from(backdrop) })
    }
}
