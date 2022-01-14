use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindScale {
    kind: NumericValue,
    axis: AxisXY,
}
impl Display for TailwindScale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        match self.axis {
            AxisXY::N => write!(f, "scale-{}", self.kind),
            AxisXY::X => write!(f, "scale-x-{}", self.kind),
            AxisXY::Y => write!(f, "scale-y-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindScale {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let scale = self.kind.get_properties(|f| (f / 100.0).to_string());
        let scale = match self.axis {
            AxisXY::N => format!("scale({})", scale),
            AxisXY::X => format!("scaleX({})", scale),
            AxisXY::Y => format!("scaleY({})", scale),
        };
        css_attributes! {
            "transform" => scale
        }
    }
}

// noinspection DuplicatedCode
impl TailwindScale {
    // https://tailwindcss.com/docs/scale
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let (rest, axis) = match pattern {
            ["x", rest @ ..] => (rest, AxisXY::X),
            ["y", rest @ ..] => (rest, AxisXY::Y),
            [..] => (pattern, AxisXY::N),
        };
        let kind = NumericValue::negative_parser("scale", |_| false)(rest, arbitrary, negative)?;
        Ok(TailwindScale { kind, axis })
    }
}
