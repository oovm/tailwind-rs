use super::*;

pub(crate) mod text_align;
pub(crate) mod text_color;
pub(crate) mod text_overflow;
pub(crate) mod text_transform;

pub fn text_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let out = match pattern {
        // https://tailwindcss.com/docs/text-align
        [s @ ("left" | "center" | "right" | "justify")] => TailwindTextAlignment::from(*s).boxed(),
        ["align", rest @ ..] => TailwindTextAlignment::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/text-overflow
        [s @ ("ellipsis" | "clip")] => TailwindTextAlignment::from(*s).boxed(),
        ["overflow", rest @ ..] => TailwindTextAlignment::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/text-transform
        ["transform", rest @ ..] => TailwindTextTransform::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/text-color
        _ => {
            let color = TailwindColor::parse(pattern, arbitrary)?;
            TailwindTextColor::from(color).boxed()
        },
    };
    Ok(out)
}
