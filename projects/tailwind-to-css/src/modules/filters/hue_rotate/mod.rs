use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindHueRotate {
    degree: IntegerOnly,
    backdrop: Backdrop,
}

impl Display for TailwindHueRotate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "hue-rotate-{}", self.degree)
    }
}

impl TailwindInstance for TailwindHueRotate {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = self.backdrop.filter();
        let value = match &self.degree {
            IntegerOnly::Number(n) => format!("hue-rotate({}%)", n),
            IntegerOnly::Arbitrary(n) => format!("hue-rotate({})", n),
        };
        css_attributes! {
            class => value
        }
    }
}

impl TailwindHueRotate {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = IntegerOnly::parser("hue-rotate")(rest, arbitrary)?;
        Ok(Self { degree: percent, backdrop: Backdrop::from(backdrop) })
    }
}
