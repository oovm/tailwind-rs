use std::fmt::Write;

use super::*;

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindScale {
    negative: bool,
    scale: usize,
    axis: Option<bool>,
}
impl Display for TailwindScale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        match self.axis {
            None => write!(f, "scale-{}", self.scale),
            Some(true) => write!(f, "scale-x-{}", self.scale),
            Some(false) => write!(f, "scale-y-{}", self.scale),
        }
    }
}

impl TailwindInstance for TailwindScale {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let scale = self.scale as f32 / 100.0;
        let scale = match self.axis {
            None => format!("scale({})", scale),
            Some(true) => format!("scaleX({})", scale),
            Some(false) => format!("scaleY({})", scale),
        };
        css_attributes! {
            "transform" => scale
        }
    }
}

impl TailwindScale {
    // https://tailwindcss.com/docs/scale
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary, axis: Option<bool>, neg: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary");
        match input {
            [n] => Ok(Self { negative: neg, scale: TailwindArbitrary::from(*n).as_integer()?, axis }),
            _ => syntax_error!("Unknown scale instructions: {}", input.join("-")),
        }
    }
}
