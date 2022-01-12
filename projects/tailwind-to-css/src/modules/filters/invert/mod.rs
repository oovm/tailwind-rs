use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindInvert {
    percent: NumericValue,
    backdrop: Backdrop,
}
impl Display for TailwindInvert {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_assert!(self.percent <= 100);
        self.backdrop.write(f)?;
        write!(f, "invert-{}", self.percent)
    }
}

impl TailwindInstance for TailwindInvert {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = self.backdrop.filter();
        let value = match &self.percent {
            NumericValue::Number(n, _) => format!("invert({}%)", n),
            NumericValue::Arbitrary(n) => format!("invert({})", n.get_properties()),
            NumericValue::Keyword(_) => unreachable!(),
        };
        css_attributes! {
            class => value
        }
    }
}

impl TailwindInvert {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = match rest {
            [] if arbitrary.is_none() => 100usize.into(),
            _ => NumericValue::positive_parser("invert")(rest, arbitrary)?,
        };
        Ok(Self { percent, backdrop: Backdrop::from(backdrop) })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = NumericValue::parse_arbitrary(arbitrary)?;
        Ok(Self { percent, backdrop: Backdrop::from(backdrop) })
    }
}
