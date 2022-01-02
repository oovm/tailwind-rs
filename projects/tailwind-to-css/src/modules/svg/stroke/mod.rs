use crate::syntax_error;

use super::*;

pub(crate) mod stroke_color;
pub(crate) mod stroke_width;

#[derive(Clone, Debug)]
pub struct TailwindStroke {}

impl TailwindStroke {
    pub fn parse(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let color = |color| TailwindStrokeColor::from(color).boxed();
        let out = match str {
            // https://tailwindcss.com/docs/text-decoration-color
            ["black"] => color(TailwindColor::Black),
            ["white"] => color(TailwindColor::White),
            ["color"] => color(TailwindColor::parse_arbitrary(arbitrary)?),
            ["color", rest] => {
                let a = TailwindArbitrary::from(*rest);
                color(TailwindColor::parse_arbitrary(&a)?)
            },
            // https://tailwindcss.com/docs/text-decoration-color
            [theme, weight] => color(TailwindColor::parse_themed(theme, weight)?),
            // https://tailwindcss.com/docs/text-decoration-thickness
            [n] => maybe_width(n)?,
            _ => return syntax_error!("Unknown decoration instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}

fn maybe_width(s: &str) -> Result<Box<dyn TailwindInstance>> {
    Ok(TailwindStrokeWidth::try_new(s)?.boxed())
}
