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
        let class = self.backdrop.filter();
        let value = match &self.percent {
            NumericValue::Number(n, _) => format!("brightness({}%)", n),
            NumericValue::Arbitrary(n) => format!("brightness({})", n.get_properties()),
            NumericValue::Keyword(_) => unreachable!(),
        };
        css_attributes! {
            class => value
        }
    }
}

impl TailwindBrightness {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = NumericValue::positive_parser("brightness")(rest, arbitrary)?;
        Ok(Self { percent, backdrop: Backdrop::from(backdrop) })
    }
}
