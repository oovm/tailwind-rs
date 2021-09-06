use super::*;

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindOpacity {
    opacity: usize,
}

impl Display for TailwindOpacity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_assert!(self.opacity <= 100);
        write!(f, "opacity-{}", self.opacity)
    }
}

impl TailwindInstance for TailwindOpacity {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        debug_assert!(self.opacity <= 100);
        let opacity = format!("{}", self.opacity as f32 / 100.0);
        css_attributes! {
            "opacity" => opacity
        }
    }
}

impl TailwindOpacity {
    /// https://tailwindcss.com/docs/opacity
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after duration");
        match input {
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Ok(Self { opacity: a.as_integer()? })
            },
            _ => syntax_error!("Unknown opacity instructions: {}", input.join("-")),
        }
    }
}
