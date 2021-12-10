#[doc = include_str!("sepia.md")]
#[derive(Clone, Debug)]
pub struct TailwindSepia {
    percent: usize,
    backdrop: bool,
}

impl Display for TailwindSepia {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_assert!(self.percent <= 100);
        if self.backdrop {
            f.write_str("backdrop-")?;
        }
        write!(f, "sepia-{}", self.percent)
    }
}

impl TailwindInstance for TailwindSepia {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        todo!()
    }
}

impl TailwindSepia {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after sepia");
        match rest {
            [n] => Ok(Self { percent: parse_integer(n)?.1, backdrop }),
            _ => syntax_error!("Unknown sepia instructions"),
        }
    }
}
