use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindGrayscale {
    percent: NumericValue,
    backdrop: Backdrop,
}
impl Display for TailwindGrayscale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_assert!(self.percent <= 100);
        self.backdrop.write(f)?;
        write!(f, "grayscale-{}", self.percent)
    }
}

impl TailwindInstance for TailwindGrayscale {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = self.backdrop.filter();
        let value = match &self.percent {
            NumericValue::Number(n, _) => format!("grayscale({}px)", n),
            NumericValue::Arbitrary(n) => format!("grayscale({})", n.get_properties()),
            NumericValue::Standard(_) => unreachable!(),
        };
        css_attributes! {
            class => value
        }
    }
}

impl TailwindGrayscale {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = match rest {
            [] if arbitrary.is_none() => 100usize.into(),
            _ => NumericValue::positive_parser("grayscale")(rest, arbitrary)?,
        };
        Ok(Self { percent, backdrop: Backdrop::from(backdrop) })
    }
}
