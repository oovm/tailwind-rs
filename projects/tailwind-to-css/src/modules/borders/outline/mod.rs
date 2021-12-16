use super::*;

pub(crate) mod outline_offset;
pub(crate) mod outline_style;
pub(crate) mod outline_width;

#[derive(Copy, Clone, Debug)]
pub struct TailwindOutline {}

impl TailwindOutline {
    /// Parse the instructions starting with `outline`.
    pub fn adapt(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/outline-style
            [] if arbitrary.is_none() => TailwindOutlineStyle::from("solid").boxed(),
            [s @ ("dashed" | "dotted" | "double" | "hidden")] => TailwindOutlineStyle::from(*s).boxed(),
            ["none"] => TailwindOutlineStyle::None.boxed(),
            ["style", rest @ ..] => TailwindOutlineStyle::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/outline-offset
            ["offset", _n] => todo!(),
            // https://tailwindcss.com/docs/outline-width
            [_n] => todo!(),
            _ => return syntax_error!("Unknown outline instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}
