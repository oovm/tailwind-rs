#[doc = include_str!("contrast.md")]
#[derive(Clone, Debug)]
pub struct TailwindContrast {
    percent: usize,
    backdrop: bool,
}

impl Display for TailwindContrast {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.backdrop {
            f.write_str("backdrop-")?;
        }
        write!(f, "contrast-{}", self.percent)
    }
}

impl TailwindInstance for TailwindContrast {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        todo!()
    }
}

impl TailwindContrast {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after contrast");
        match rest {
            [n] => Ok(Self { percent: parse_integer(n)?.1, backdrop }),
            _ => syntax_error!("Unknown contrast instructions"),
        }
    }
}
