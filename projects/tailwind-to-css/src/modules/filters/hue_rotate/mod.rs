use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindHueRotate {
    degree: NumericValue,
    backdrop: Backdrop,
}

impl Display for TailwindHueRotate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "hue-rotate-{}", self.degree)
    }
}

impl TailwindInstance for TailwindHueRotate {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = self.backdrop.filter();
        let value = match &self.degree {
            NumericValue::Number(n, _) => format!("hue-rotate({}%)", n),
            NumericValue::Arbitrary(n) => format!("hue-rotate({})", n.get_properties()),
            NumericValue::Keyword(_) => unreachable!(),
        };
        css_attributes! {
            class => value
        }
    }
}

impl TailwindHueRotate {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = NumericValue::positive_parser("hue-rotate")(rest, arbitrary)?;
        Ok(Self { degree: percent, backdrop: Backdrop::from(backdrop) })
    }
}
