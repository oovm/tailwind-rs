use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRotate {
    kind: NumericValue,
}

impl Display for TailwindRotate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        self.kind.write_class(f, "rotate-")
    }
}

impl TailwindInstance for TailwindRotate {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let rotate = match &self.kind {
            NumericValue::Number(n, _) => format!("rotate({}deg)", n),
            NumericValue::Arbitrary(s) => format!("rotate({})", s.get_properties()),
            NumericValue::Standard(_) => unreachable!(),
        };
        css_attributes! {
            "transform" => rotate
        }
    }
}

impl TailwindRotate {
    // <https://tailwindcss.com/docs/rotate>
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let kind = NumericValue::negative_parser("scale")(input, arbitrary, negative)?;
        Ok(Self { kind })
    }
}
