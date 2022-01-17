use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBlur {
    px: NumericValue,
    backdrop: Backdrop,
}
impl Display for TailwindBlur {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "blur-{}", self.px)
    }
}

impl TailwindInstance for TailwindBlur {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let n = self.px.get_properties(|f| format!("{}px", f));
        self.backdrop.get_filter(format!("blur({})", n))
    }
}

impl TailwindBlur {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let px = match rest {
            [] if arbitrary.is_none() => 8u32.into(),
            _ => NumericValue::positive_parser("blur", |_| false)(rest, arbitrary)?,
        };
        Ok(Self { px, backdrop: Backdrop::from(backdrop) })
    }
}
