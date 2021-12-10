use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBrightness {
    percent: usize,
    backdrop: bool,
}

impl Display for TailwindBrightness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.backdrop {
            f.write_str("backdrop-")?;
        }
        write!(f, "brightness-{}", self.percent)
    }
}

impl TailwindInstance for TailwindBrightness {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let filter = match self.backdrop {
            true => "backdrop-filter",
            false => "filter",
        };
        let brightness = format!("brightness({})", self.percent as f32 / 100.0);
        css_attributes! {
            filter => brightness
        }
    }
}

impl TailwindBrightness {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after brightness");
        match rest {
            [n] => Ok(Self { percent: parse_integer(n)?.1, backdrop }),
            _ => syntax_error!("Unknown brightness instructions"),
        }
    }
}
