use super::*;

pub use self::{
    border_color::TailwindBorderColor, border_radius::TailwindRounded, border_style::TailwindBorderStyle,
    border_width::TailwindBorderWidth,
};

mod border_color;
mod border_radius;
mod border_style;
mod border_width;

///
#[derive(Debug, Clone)]
pub struct TailwindBorder {
    arbitrary: String,
}

impl TailwindBorder {
    pub fn parse(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
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
            [] => TailwindBorder { arbitrary: arbitrary.to_string() }.boxed(),

            _ => return syntax_error!("Unknown border instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}

impl Display for TailwindBorder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindBorder {}
