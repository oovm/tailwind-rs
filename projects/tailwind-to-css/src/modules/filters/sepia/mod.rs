use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindSepia {
    percent: NumericValue,
    backdrop: Backdrop,
}
impl Display for TailwindSepia {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "sepia-{}", self.percent)
    }
}

impl TailwindInstance for TailwindSepia {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let n = self.percent.get_properties(|f| format!("{}%", f));
        self.backdrop.get_filter(format!("sepia({})", n))
    }
}

impl TailwindSepia {
    /// <https://tailwindcss.com/docs/sepia>
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = match rest {
            [] if arbitrary.is_none() => 100i32.into(),
            _ => NumericValue::positive_parser("sepia", |_| false)(rest, arbitrary)?,
        };
        Ok(Self { percent, backdrop: Backdrop::from(backdrop) })
    }
}
