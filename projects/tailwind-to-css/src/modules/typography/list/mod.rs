use super::*;

pub(crate) mod list_position;
pub(crate) mod list_type;

pub(crate) fn list_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let out = match str {
        // https://tailwindcss.com/docs/list-style-position
        [s @ ("inside" | "outside")] => TailwindListPosition::from(*s).boxed(),
        ["position", rest @ ..] => TailwindListPosition::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/list-style-type
        _ => TailwindListStyle::parse(str, arbitrary)?.boxed(),
    };
    Ok(out)
}
