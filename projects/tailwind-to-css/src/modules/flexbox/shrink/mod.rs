use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailWindShrink {
    shrink: usize,
}

impl Display for TailWindShrink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "shrink-{}", self.shrink)
    }
}

impl TailwindInstance for TailWindShrink {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "flex-shrink" => self.shrink
        }
    }
}

impl TailWindShrink {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after flex-shrink");
        match pattern {
            [] => Ok(Self { shrink: 0 }),
            [n] => Ok(Self { shrink: parse_integer(n)?.1 }),
            _ => syntax_error!("Unknown shrink instructions: {}", pattern.join("-")),
        }
    }
}
