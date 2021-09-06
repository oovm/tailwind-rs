use super::*;

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindDuration {
    ms: usize,
}

impl Display for TailwindDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "duration-{}", self.ms)
    }
}

impl TailwindInstance for TailwindDuration {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "transition-duration" => format!("{}ms", self.ms)
        }
    }
}

impl TailwindDuration {
    /// https://tailwindcss.com/docs/transition-duration
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after duration");
        match input {
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Ok(Self { ms: a.as_integer()? })
            }
            _ => syntax_error!("Unknown duration instructions: {}", input.join("-")),
        }
    }
}
