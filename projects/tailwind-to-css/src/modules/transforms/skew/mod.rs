use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindSkew {
    negative: Negative,
    axis: AxisXY,
    degree: IntegerOnly,
}

impl Display for TailwindSkew {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.negative.write(f)?;
        match self.axis {
            AxisXY::X => write!(f, "skew-x-{}", self.degree),
            AxisXY::Y => write!(f, "skew-y-{}", self.degree),
            AxisXY::N => write!(f, "skew-{}", self.degree),
        }
    }
}

impl TailwindInstance for TailwindSkew {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let skew = match self.axis {
            AxisXY::X => format!("skewX({}deg)", self.degree),
            AxisXY::Y => format!("skewY({}deg)", self.degree),
            AxisXY::N => format!("skew({}deg)", self.degree),
        };
        css_attributes! {
            "transform" => skew
        }
    }
}

impl TailwindSkew {
    // <https://tailwindcss.com/docs/skew>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let (axis, rest) = match pattern {
            ["x", rest @ ..] => (AxisXY::X, rest),
            ["y", rest @ ..] => (AxisXY::Y, rest),
            _ => (AxisXY::X, pattern),
        };
        let degree = IntegerOnly::parser("skew")(rest, arbitrary)?;
        Ok(Self { negative: negative.into(), degree, axis })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, negative: Negative, axis: AxisXY) -> Result<Self> {
        let degree = IntegerOnly::parse_arbitrary(arbitrary)?;
        Ok(Self { negative: negative.into(), degree, axis })
    }
}
