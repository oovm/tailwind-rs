use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindTranslate {
    axis: AxisXY,
    kind: UnitValue,
}

impl Display for TailwindTranslate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        match self.axis {
            AxisXY::X => write!(f, "translate-x-{}", self.kind),
            AxisXY::Y => write!(f, "translate-y-{}", self.kind),
            AxisXY::N => write!(f, "translate-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindTranslate {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let size = self.kind.get_properties(|n| format!("{}rem", n / 4.0));
        let transform = match self.axis {
            AxisXY::X => format!("translateX({})", size),
            AxisXY::Y => format!("translateY({})", size),
            AxisXY::N => format!("translate({})", size),
        };
        css_attributes! {
            "transform" => transform,
        }
    }
}

// noinspection DuplicatedCode
impl TailwindTranslate {
    /// <https://tailwindcss.com/docs/translate>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let (axis, rest) = AxisXY::split_xyn(pattern);
        let kind = match rest {
            ["px"] => UnitValue::px(1.0),
            ["full"] => UnitValue::radio(1, 1),
            _ => UnitValue::negative_parser("translate", |_| false, true, false, false)(rest, arbitrary, negative)?,
        };
        Ok(Self { kind, axis })
    }
}
