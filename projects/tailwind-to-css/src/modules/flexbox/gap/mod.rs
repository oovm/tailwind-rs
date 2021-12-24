use super::*;
use crate::AxisXY;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindGap {
    size: LengthUnit,
    axis: AxisXY,
}

impl Display for TailwindGap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            AxisXY::N => write!(f, "gap-{}", self.size.get_class_arbitrary()),
            AxisXY::X => write!(f, "gap-x-{}", self.size.get_class_arbitrary()),
            AxisXY::Y => write!(f, "gap-y-{}", self.size.get_class_arbitrary()),
        }
    }
}

impl TailwindInstance for TailwindGap {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = match self.axis {
            AxisXY::N => "gap",
            AxisXY::X => "column-gap",
            AxisXY::Y => "row-gap",
        };
        css_attributes! {
            class => self.size.get_properties()
        }
    }
}

impl TailwindGap {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            ["x", rest @ ..] => Ok(Self { size: parse_size(rest, arbitrary)?, axis: AxisXY::X }),
            ["y", rest @ ..] => Ok(Self { size: parse_size(rest, arbitrary)?, axis: AxisXY::Y }),
            _ => Ok(Self { size: parse_size(pattern, arbitrary)?, axis: AxisXY::N }),
        }
    }
}

fn parse_size(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<LengthUnit> {
    let size = match pattern {
        [] => arbitrary.as_length()?,
        ["px"] => LengthUnit::px(1.0),
        [n] => {
            let a = TailwindArbitrary::from(*n);
            LengthUnit::rem(a.as_float()? / 4.0)
        },
        _ => return syntax_error!("Unknown gap instructions"),
    };
    Ok(size)
}
