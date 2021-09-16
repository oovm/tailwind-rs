use crate::{
    modules::typography::decoration::{style::TailwindDecorationStyle, thickness::TailwindDecorationThickness},
    LengthUnit, TailwindColor,
};

use super::*;

pub use self::{color::TailwindDecorationColor, line::TailwindDecorationLine};

mod color;
mod line;
mod style;
#[cfg(test)]
mod test;
mod thickness;

#[derive(Debug, Clone)]
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
            ["color", rest] => {
                let a = TailwindArbitrary::from(*rest);
                color(TailwindColor::parse_arbitrary(&a)?)
            },
            // https://tailwindcss.com/docs/text-decoration-color
            [theme, weight] => color(TailwindColor::parse_themed(theme, weight)?),
            // https://tailwindcss.com/docs/text-decoration-thickness
            [n] => TailwindDecorationThickness::parse(n)?.boxed(),
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
        write!(f, "decoration-[{}]", self.arbitrary)
    }
}

impl TailwindInstance for TailwindDecoration {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "text-decoration" => self.arbitrary
        }
    }
}
