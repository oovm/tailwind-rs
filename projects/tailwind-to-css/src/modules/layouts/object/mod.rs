mod object_fit;
mod object_position;
#[cfg(test)]
mod test;

pub use self::{object_fit::TailwindObjectFit, object_position::TailwindObjectPosition};
use super::*;

pub fn object_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let out = match pattern {
        // https://tailwindcss.com/docs/object-fit
        [s @ ("contain" | "cover" | "fill" | "none")] => TailwindObjectFit::from(*s).boxed(),
        ["scale", "down"] => TailwindObjectFit::from("scale-down").boxed(),
        // https://tailwindcss.com/docs/object-position
        _ => TailwindObjectPosition::parse(pattern, arbitrary)?.boxed(),
    };
    Ok(out)
}
