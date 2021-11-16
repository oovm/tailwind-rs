use super::*;

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindRotate {
    negative: bool,
    degree: usize,
}
impl Display for TailwindRotate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        write!(f, "rotate-{}", self.degree)
    }
}

impl TailwindInstance for TailwindRotate {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let rotate = format!("rotate({}deg)", self.degree);
        css_attributes! {
            "transform" => rotate
        }
    }
}

impl TailwindRotate {
    // https://tailwindcss.com/docs/rotate
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary, negative: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary");
        match input {
            [n] => Ok(Self { negative, degree: TailwindArbitrary::from(*n).as_integer()? }),
            _ => syntax_error!("Unknown rotate instructions: {}", input.join("-")),
        }
    }
}
