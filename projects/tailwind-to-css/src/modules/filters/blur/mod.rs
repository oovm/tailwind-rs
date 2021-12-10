use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBlur {
    px: IntegerOnly,
    backdrop: Backdrop,
}
impl Display for TailwindBlur {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "blur-{}", self.px)
    }
}

impl TailwindInstance for TailwindBlur {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = self.backdrop.filter();
        match &self.px {
            IntegerOnly::Number(n) => {
                format!("blur({}px)", self.px)
            },
            IntegerOnly::Keywords(n) => {
                format!("blur({}px)", self.px)
            },
            IntegerOnly::Arbitrary(n) => {
                format!("blur({}px)", self.px)
            },
        }

        css_attributes! {
            class => format!("blur({}px)", self.px)
        }
    }
}

impl TailwindBlur {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after blur");
        let px = match rest {
            ["none"] => 0,
            ["sm"] => 4,
            ["base"] => 8,
            [] => {
                debug_assert!(arbitrary.is_none());
                8
            },
            ["md"] => 12,
            ["lg"] => 16,
            ["xl"] => 24,
            ["2xl"] => 40,
            ["3xl"] => 64,
            [n] => parse_i_px_maybe(n)?.1,
            _ => return syntax_error!("Unknown blur instructions"),
        };
        Ok(Self { px: IntegerOnly::from(px), backdrop: Backdrop::from(backdrop) })
    }
}
