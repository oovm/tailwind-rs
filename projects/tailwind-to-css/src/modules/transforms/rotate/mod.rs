use super::*;

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindRotate {
    neg: bool,
    deg: usize,
}
impl Display for TailwindRotate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rotate-{}", self.deg)
    }
}

impl TailwindInstance for TailwindRotate {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let rotate = format!("rotate({}deg)", self.deg);
        css_attributes! {
            "transform" => rotate
        }
    }
}

impl TailwindRotate {
    // https://tailwindcss.com/docs/rotate
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary, neg: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary");
        match input {
            [n] => Ok(Self { neg, deg: TailwindArbitrary::from(*n).as_integer()? }),
            _ => syntax_error!("Unknown rotate instructions: {}", input.join("-")),
        }
    }
}
