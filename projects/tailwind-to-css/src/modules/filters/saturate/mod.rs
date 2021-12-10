use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindSaturate {
    percent: usize,
    backdrop: bool,
}

impl Display for TailwindSaturate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.backdrop {
            f.write_str("backdrop-")?;
        }
        write!(f, "saturate-{}", self.percent)
    }
}

impl TailwindInstance for TailwindSaturate {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        todo!()
    }
}

impl TailwindSaturate {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after saturate");
        match rest {
            [n] => Ok(Self { percent: parse_integer(n)?.1, backdrop }),
            _ => syntax_error!("Unknown saturate instructions"),
        }
    }
}
