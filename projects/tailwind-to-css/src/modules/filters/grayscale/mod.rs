use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindGrayscale {
    percent: IntegerOnly,
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
            IntegerOnly::Number(n) => format!("grayscale({}px)", n),
            IntegerOnly::Arbitrary(n) => format!("grayscale({})", n.get_properties()),
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
            _ => IntegerOnly::parser("grayscale")(rest, arbitrary)?,
        };
        Ok(Self { percent, backdrop: Backdrop::from(backdrop) })
    }
}
