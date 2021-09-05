use super::*;

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindDelay {
    ms: usize,
}

impl Display for TailwindDelay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "delay-{}", self.ms)
    }
}

impl TailwindInstance for TailwindDelay {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "transition-delay" => format!("{}ms", self.ms)
        }
    }
}

impl TailwindDelay {
    /// https://tailwindcss.com/docs/transition-delay
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after duration");
        match input {
            [n] => Ok(Self { ms: parse_usize(n)? }),
            _ => syntax_error!("Unknown delay instructions: {}", input.join("-")),
        }
    }
}
