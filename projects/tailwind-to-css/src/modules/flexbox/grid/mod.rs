use super::*;

pub(crate) mod grid_auto;
pub(crate) mod grid_cols;
pub(crate) mod grid_flow;
pub(crate) mod grid_rows;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone, Copy)]
pub struct TailwindGrid {}

impl TailwindGrid {
    pub fn adapt(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/grid-template-rows
            ["rows", rest @ ..] => TailwindGridRows::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/grid-template-columns
            ["cols", rest @ ..] => TailwindGridColumns::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/grid-auto-flow
            ["flow", rest @ ..] => TailwindGridFlow::parse(rest, arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown list instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}

#[derive(Debug, Clone)]
enum GridTemplate {
    None,
    Unit(usize),
    Arbitrary(TailwindArbitrary),
}

impl Display for GridTemplate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GridTemplate::None => write!(f, "none"),
            GridTemplate::Unit(s) => write!(f, "{}", s),
            GridTemplate::Arbitrary(s) => s.write(f),
        }
    }
}
impl GridTemplate {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["none"] => Self::None,
            [n] => Self::Unit(TailwindArbitrary::from(*n).as_integer()?),
            _ => Self::parse_arbitrary(arbitrary)?,
        };
        Ok(kind)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
    pub fn get_properties(&self) -> String {
        match self {
            GridTemplate::None => "none".to_string(),
            GridTemplate::Unit(s) => format!("repeat({},minmax(0,1fr))", s),
            GridTemplate::Arbitrary(s) => s.get_properties(),
        }
    }
}
