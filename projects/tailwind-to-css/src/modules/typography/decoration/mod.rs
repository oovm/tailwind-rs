use crate::{
    modules::typography::decoration::{style::TailwindDecorationStyle, thickness::TailwindDecorationThickness},
    LengthUnit, TailwindColor,
};

use super::*;

pub use self::{color::TailwindDecorationColor, line::TailwindDecorationLine};

mod color;
mod line;
mod style;
mod thickness;

// decoration-inherit	text-decoration-color: inherit;
// decoration-current	text-decoration-color: currentColor;
// decoration-transparent	text-decoration-color: transparent;
// decoration-black	text-decoration-color: #000;
// decoration-white	text-decoration-color: #fff;
// decoration-slate-50	text-decoration-color: #f8fafc;

// decoration-auto	text-decoration-thickness: auto;
// decoration-from-font	text-decoration-thickness: from-font;
// decoration-0	text-decoration-thickness: 0px;
// decoration-1	text-decoration-thickness: 1px;
// decoration-2	text-decoration-thickness: 2px;
// decoration-4	text-decoration-thickness: 4px;
// decoration-8	text-decoration-thickness: 8px;

pub struct TailwindDecoration {
    arbitrary: String,
}

impl TailwindDecoration {
    pub fn parse(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let style = |kind| TailwindDecorationStyle::from(kind).boxed();
        let color = |color| TailwindDecorationColor::from(color).boxed();
        let out = match str {
            // https://tailwindcss.com/docs/text-decoration-style
            [s @ ("solid" | "double" | "dotted" | "dashed" | "wavy")] => style(*s),
            // https://tailwindcss.com/docs/text-decoration-thickness
            ["auto"] => TailwindDecorationThickness::Auto.boxed(),
            ["from", "font"] => TailwindDecorationThickness::FromFont.boxed(),
            // https://tailwindcss.com/docs/text-decoration-color
            ["current"] => color(TailwindColor::Current),
            ["transparent"] => color(TailwindColor::Transparent),
            ["inherit"] => color(TailwindColor::Inherit),
            ["black"] => color(TailwindColor::Black),
            ["white"] => color(TailwindColor::White),
            ["color"] => {
                debug_assert!(arbitrary.is_some());
                color(TailwindColor::parse_arbitrary(arbitrary)?)
            },
            // https://tailwindcss.com/docs/text-decoration-color
            [theme, weight] => color(TailwindColor::parse_themed(theme, weight)?),
            [] => {
                debug_assert!(arbitrary.is_some());
                TailwindDecoration { arbitrary: arbitrary.to_string() }.boxed()
            },
            _ => return syntax_error!("Unknown decoration instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}

impl Display for TailwindDecoration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindDecoration {}
