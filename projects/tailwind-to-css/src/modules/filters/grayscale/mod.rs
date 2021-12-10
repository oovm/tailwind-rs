#[doc = include_str!("grayscale.md")]
#[derive(Clone, Debug)]
pub struct TailwindGrayscale {
    percent: usize,
    backdrop: bool,
}

impl Display for TailwindGrayscale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_assert!(self.percent <= 100);
        if self.backdrop {
            f.write_str("backdrop-")?;
        }
        write!(f, "grayscale-{}", self.percent)
    }
}

impl TailwindInstance for TailwindGrayscale {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        todo!()
    }
}

impl TailwindGrayscale {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after grayscale");
        match rest {
            [n] => Ok(Self { percent: parse_integer(n)?.1, backdrop }),
            _ => syntax_error!("Unknown grayscale instructions"),
        }
    }
}
