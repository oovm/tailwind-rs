use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailWindGrow {
    grow: usize,
}

impl Display for TailWindGrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "grow-{}", self.grow)
    }
}

impl TailwindInstance for TailWindGrow {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "flex-grow" => self.grow
        }
    }
}

impl TailWindGrow {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after flex-grow");
        match pattern {
            [] => Ok(Self { grow: 0 }),
            [n] => Ok(Self { grow: parse_integer(n)?.1 }),
            _ => syntax_error!("Unknown grow instructions: {}", pattern.join("-")),
        }
    }
}
