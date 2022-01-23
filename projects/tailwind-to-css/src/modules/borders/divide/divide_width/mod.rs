use crate::NumericValue;

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindDivideWidth {
    axis: AxisXY,
    kind: NumericValue,
}

impl Display for TailwindDivideWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.axis.write_xy(f, "divide-", &self.kind)
    }
}

impl TailwindInstance for TailwindDivideWidth {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        match self.axis {
            AxisXY::X => css_attributes! {
                "border-right-width" => format!("{}px", self.kind),
                "border-left-width" => "0"
            },
            AxisXY::Y => css_attributes! {
                "border-top-width" => "0",
                "border-bottom-width" => format!("{}px", self.kind)
            },
            AxisXY::N => unreachable!(),
        }
    }
}

impl TailwindDivideWidth {
    /// https://tailwindcss.com/docs/divide-width
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary, axis: bool) -> Result<Self> {
        let kind = NumericValue::positive_parser("divide-width", Self::check_valid)(input, arbitrary)?;
        Ok(Self { axis: AxisXY::from(axis), kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-style#syntax>
    pub fn check_valid(mode: &str) -> bool {
        [
            "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "ridge", "inset", "outset", "inherit",
            "initial", "revert", "unset",
        ]
        .contains(&mode)
    }
}
