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
        let value = format!("blur({})", n);
        match self.backdrop.0 {
            true => css_attributes! {
                "backdrop-filter" => value
            },
            false => css_attributes! {
                "filter" => value
            },
        }
    }
}

impl TailwindBlur {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let px = match rest {
            ["none"] => 0u32.into(),
            ["sm"] => 4u32.into(),
            ["base"] => 8u32.into(),
            [] if arbitrary.is_none() => 8u32.into(),
            ["md"] => 12u32.into(),
            ["lg"] => 16u32.into(),
            ["xl"] => 24u32.into(),
            ["2xl"] => 40u32.into(),
            ["3xl"] => 64u32.into(),
            _ => NumericValue::positive_parser("blur", |_| false)(rest, arbitrary)?,
        };
        Ok(Self { px, backdrop: Backdrop::from(backdrop) })
    }
}
