use super::*;

pub(crate) mod ring_color;
pub(crate) mod ring_inset;
pub(crate) mod ring_offset_color;
pub(crate) mod ring_offset_width;
pub(crate) mod ring_width;

#[derive(Copy, Clone, Debug, Default)]
pub struct TailwindRing {}

impl TailwindRing {
    pub fn adapt(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/ring-offset-width
            ["offset", rest @ ..] => TailwindRingOffsetWidth::parse(rest, arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown ring instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}
