use crate::StandardValue;

use super::*;

pub(crate) mod border_color;
pub(crate) mod border_radius;
pub(crate) mod border_style;
pub(crate) mod border_width;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindBorder {
    kind: StandardValue,
}

impl TailwindBorder {
    /// Parse the instructions starting with `border`.
    pub fn adapt(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let color = |color| TailwindBorderColor::from(color).boxed();
        let out = match str {
            // https://tailwindcss.com/docs/border-style
            [s @ ("solid" | "dashed" | "dotted" | "double" | "hidden" | "none")] => TailwindBorderStyle::from(*s).boxed(),
            // https://tailwindcss.com/docs/border-collapse
            [s @ ("collapse" | "separate")] => TailwindBorderCollapse::from(*s).boxed(),
            // https://tailwindcss.com/docs/border-color
            ["inherit"] => color(TailwindColor::Inherit),
            ["current"] => color(TailwindColor::Current),
            ["transparent"] => color(TailwindColor::Transparent),
            ["black"] => color(TailwindColor::Black),
            ["white"] => color(TailwindColor::White),
            [] => TailwindBorder { kind: StandardValue::parse_arbitrary(arbitrary)? }.boxed(),
            _ => return syntax_error!("Unknown border instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}

impl Display for TailwindBorder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "border-[{}]", self.kind)
    }
}

impl TailwindInstance for TailwindBorder {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        todo!()
    }
}
