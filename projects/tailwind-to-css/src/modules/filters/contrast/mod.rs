use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindContrast {
    percent: IntegerOnly,
    backdrop: Backdrop,
}

impl Display for TailwindContrast {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "contrast-{}", self.percent)
    }
}

impl TailwindInstance for TailwindContrast {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = self.backdrop.filter();
        let value = match &self.percent {
            IntegerOnly::Number(n) => format!("contrast({}%)", n),
            IntegerOnly::Arbitrary(n) => format!("contrast({})", n),
        };
        css_attributes! {
            class => value
        }
    }
}

impl TailwindContrast {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = IntegerOnly::parser("contrast")(rest, arbitrary)?;
        Ok(Self { percent, backdrop: Backdrop::from(backdrop) })
    }
}
