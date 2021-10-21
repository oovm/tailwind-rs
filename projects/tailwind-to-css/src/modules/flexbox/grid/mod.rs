use super::*;

pub(crate) mod grid_auto;
pub(crate) mod grid_flow;
pub(crate) mod grid_template;

///
#[derive(Debug, Clone, Copy)]
pub struct TailwindGrid {}

impl TailwindGrid {
    pub fn parse(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after place");
        let out = match str {
            // https://tailwindcss.com/docs/grid-template-rows
            ["rows", rest @ ..] => TailwindGridTemplate::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/grid-template-columns
            ["cols", rest @ ..] => TailwindGridTemplate::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/grid-auto-flow
            ["flow", rest @ ..] => TailwindGridFlow::parse(rest, arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown list instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}
