#[doc = include_str!("invert.md")]
#[derive(Clone, Debug)]
pub struct TailwindInvert {
    percent: usize,
    backdrop: bool,
}

impl Display for TailwindInvert {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_assert!(self.percent <= 100);
        if self.backdrop {
            f.write_str("backdrop-")?;
        }
        write!(f, "invert-{}", self.percent)
    }
}

impl TailwindInstance for TailwindInvert {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        todo!()
    }
}

impl TailwindInvert {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden invert in grayscale");
        match rest {
            [n] => Ok(Self { percent: parse_integer(n)?.1, backdrop }),
            _ => syntax_error!("Unknown invert instructions"),
        }
    }
}
