use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindInset {
    axis: AxisXY,
    kind: UnitValue,
}

impl Display for TailwindInset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        self.axis.write_xyn(f, "inset-", &self.kind)
    }
}

impl TailwindInstance for TailwindInset {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let size = self.kind.get_properties_rem();
        match self.axis {
            AxisXY::X => css_attributes! {
                "right" => &size,
                "left" => &size,
            },
            AxisXY::Y => css_attributes! {
                "top" => &size,
                "bottom" => &size,
            },
            AxisXY::N => css_attributes! {
                "top" => &size,
                "right" => &size,
                "bottom" => &size,
                "left" => &size,
            },
        }
    }
}

impl TailwindInset {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let (axis, rest) = AxisXY::split_xyn(pattern);
        let kind = get_kind_px_full_auto_fact("inset", rest, arbitrary, negative)?;
        Ok(Self { axis, kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/inset#syntax>
    pub fn check_valid(mode: &str) -> bool {
        ["auto", "inherit", "initial", "revert", "unset"].contains(&mode)
    }
}
