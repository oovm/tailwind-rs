use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindSaturate {
    percent: IntegerOnly,
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
        let class = self.backdrop.filter();
        let value = match &self.percent {
            IntegerOnly::Number(n) => format!("saturate({}%)", n),
            IntegerOnly::Arbitrary(n) => format!("saturate({})", n),
        };
        css_attributes! {
            class => value
        }
    }
}

impl TailwindSaturate {
    /// <https://tailwindcss.com/docs/saturate>
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = IntegerOnly::parser("saturate")(rest, arbitrary)?;
        Ok(Self { percent, backdrop: Backdrop::from(backdrop) })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = IntegerOnly::parse_arbitrary(arbitrary)?;
        Ok(Self { percent, backdrop: Backdrop::from(backdrop) })
    }
}
