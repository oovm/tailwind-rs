#[doc = include_str!("grayscale.md")]
#[derive(Clone, Debug)]
pub struct TailwindHueRotate {
    deg: usize,
    backdrop: bool,
}

impl Display for TailwindHueRotate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.backdrop {
            f.write_str("backdrop-")?;
        }
        write!(f, "hue-rotate-{}", self.deg)
    }
}

impl TailwindInstance for TailwindHueRotate {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        todo!()
    }
}

impl TailwindHueRotate {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after hue-rotate");
        match rest {
            [n] => Ok(Self { deg: parse_integer(n)?.1, backdrop }),
            _ => syntax_error!("Unknown hue-rotate instructions"),
        }
    }
}
