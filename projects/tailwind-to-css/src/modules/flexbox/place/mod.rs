pub(crate) mod place_content;
pub(crate) mod place_item;
pub(crate) mod place_self;

use super::*;

#[derive(Debug, Copy, Clone)]
pub struct TailwindPlace {}

impl TailwindPlace {
    pub fn adapt(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/place-content
            ["content", rest @ ..] => TailwindPlaceContent::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/place-items
            ["items", rest @ ..] => TailwindPlaceItems::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/place-self
            ["self", rest @ ..] => TailwindPlaceSelf::parse(rest, arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown place instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}
