use super::*;

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindSkew {
    neg: bool,
    deg: usize,
    axis: bool,
}

impl Display for TailwindSkew {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.neg {
            f.write_char('-')?
        }
        match self.axis {
            true => write!(f, "skew-x-{}", self.deg),
            false => write!(f, "skew-y-{}", self.deg),
        }
    }
}

impl TailwindInstance for TailwindSkew {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let skew = match self.axis {
            true => format!("skewX({}deg)", self.deg),
            false => format!("skewY({}deg)", self.deg),
        };
        css_attributes! {
            "transform" => skew
        }
    }
}

impl TailwindSkew {
    // https://tailwindcss.com/docs/skew
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, neg: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after skew");
        match pattern {
            ["x", n] => Ok(Self { neg, deg: TailwindArbitrary::from(*n).as_integer()?, axis: true }),
            ["y", n] => Ok(Self { neg, deg: TailwindArbitrary::from(*n).as_integer()?, axis: false }),
            _ => syntax_error!("Unknown skew instructions: {}", pattern.join("-")),
        }
    }
}
