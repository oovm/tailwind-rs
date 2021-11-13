use super::*;

pub(crate) mod flex_direction;
pub(crate) mod flex_wrap;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindFlex {
    None,
    Inherit,
    Auto { grow: usize, shrink: usize },
    Percent { grow: usize, shrink: usize, basis: usize },
}

impl Display for TailwindFlex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("flex-")?;
        match self {
            Self::None => f.write_str("none"),
            Self::Inherit => {
                todo!()
            },
            Self::Auto { .. } => {
                todo!()
            },
            Self::Percent { .. } => {
                todo!()
            },
        }
    }
}

impl TailwindInstance for TailwindFlex {}

impl TailwindFlex {
    pub fn parse(flex: &str) -> Result<Self> {
        let n = parse_integer(flex)?.1;
        Ok(Self::Percent { grow: n, shrink: n, basis: 0 })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}

#[inline]
pub fn flex_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let out = match str {
        // https://tailwindcss.com/docs/display#flex
        // `[]` => This won't happen
        // https://tailwindcss.com/docs/flex#arbitrary-values
        [] => TailwindFlex::parse_arbitrary(arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/flex-direction
        ["row"] => TailwindFlexDirection::from("row").boxed(),
        ["row", "reverse"] => TailwindFlexDirection::from("row-reverse").boxed(),
        ["col"] => TailwindFlexDirection::from("column").boxed(),
        ["col", "reverse"] => TailwindFlexDirection::from("column-reverse").boxed(),
        ["direction", rest @ ..] => TailwindFlexDirection::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/flex-wrap
        ["wrap"] => TailwindFlexWrap::from("wrap").boxed(),
        ["wrap", "reverse"] => TailwindFlexWrap::from("wrap-reverse").boxed(),
        ["nowrap"] => TailwindFlexWrap::from("nowrap").boxed(),
        // https://tailwindcss.com/docs/flex
        ["auto"] => TailwindFlex::Inherit.boxed(),
        ["initial"] => TailwindFlex::Inherit.boxed(),
        ["none"] => TailwindFlex::None.boxed(),
        [n] => TailwindFlex::parse(n)?.boxed(),
        _ => return syntax_error!("Unknown box instructions: {}", str.join("-")),
    };
    Ok(out)
}
