use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindSkew {
    axis: AxisXY,
    kind: UnitValue,
}

impl Display for TailwindSkew {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        self.axis.write_xyn(f, "skew-", &self.kind)
    }
}

impl TailwindInstance for TailwindSkew {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let deg = self.kind.get_properties(|n| format!("{}deg", n));
        let transform = match self.axis {
            AxisXY::X => format!("skewX({})", deg),
            AxisXY::Y => format!("skewY({})", deg),
            AxisXY::N => format!("skew({})", deg),
        };
        css_attributes! {
            "transform" => transform,
        }
    }
}

impl TailwindSkew {
    // <https://tailwindcss.com/docs/skew>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let (axis, rest) = AxisXY::split_xyn(pattern);
        let kind = UnitValue::negative_parser("skew", |_| false, false, false, false)(rest, arbitrary, negative)?;
        Ok(Self { kind, axis })
    }
}
