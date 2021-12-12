use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBrightness {
    percent: IntegerOnly,
    backdrop: Backdrop,
}

impl Display for TailwindBrightness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "brightness-{}", self.percent)
    }
}

impl TailwindInstance for TailwindBrightness {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = self.backdrop.filter();
        let value = match &self.percent {
            IntegerOnly::Number(n) => format!("brightness({}%)", n),
            IntegerOnly::Arbitrary(n) => format!("brightness({})", n),
        };
        css_attributes! {
            class => value
        }
    }
}

impl TailwindBrightness {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = IntegerOnly::parser("brightness")(rest, arbitrary)?;
        Ok(Self { percent, backdrop: Backdrop::from(backdrop) })
    }
}
