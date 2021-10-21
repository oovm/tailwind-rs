use super::*;

pub(crate) mod list_position;
pub(crate) mod list_type;

#[derive(Copy, Clone, Debug)]
pub struct TailwindList {}

impl TailwindList {
    pub fn parse(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/list-style-type
            [s @ ("none" | "disc" | "decimal")] => TailwindListStyle::from(*s).boxed(),
            ["type", rest @ ..] => TailwindListStyle::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/list-style-position
            [s @ ("inside" | "outside")] => TailwindListPosition::from(*s).boxed(),
            ["position", rest @ ..] => TailwindListPosition::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/list-style-type#arbitrary-values
            [] => TailwindListStyle::parse_arbitrary(arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown list instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}
