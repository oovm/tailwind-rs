use super::*;

pub use self::{object_fit::TailwindObjectFit, object_position::TailwindObjectPosition};

mod object_fit;
mod object_position;

pub(crate) fn object_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let joint = pattern.join("-");
    let out = match pattern {
        // https://tailwindcss.com/docs/object-fit
        ["fit", rest @ ..] => TailwindObjectFit::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/object-position
        ["position", rest @ ..] => TailwindObjectPosition::parse(rest, arbitrary)?.boxed(),
        // Flexible parsing pattern
        _ if TailwindObjectFit::check_valid(&joint) => TailwindObjectFit::from(joint).boxed(),
        [..] => TailwindObjectPosition::parse(pattern, arbitrary)?.boxed(),
    };
    Ok(out)
}
