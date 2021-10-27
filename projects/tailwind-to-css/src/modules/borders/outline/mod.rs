use super::*;

pub use self::{
    outline_offset::TailwindOutlineOffset, outline_style::TailwindOutlineStyle, outline_width::TailwindOutlineWidth,
};

mod outline_offset;
mod outline_style;
mod outline_width;

#[derive(Copy, Clone, Debug)]
pub struct TailwindOutline {}

#[inline]
pub fn outline_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
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
