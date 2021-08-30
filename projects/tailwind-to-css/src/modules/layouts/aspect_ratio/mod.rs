use super::*;

#[derive(Copy, Clone, Debug)]
enum AspectKind {
    Auto,
    Radio(usize, usize),
    Global(CssBehavior),
}

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindAspect {
    kind: AspectKind,
}
impl Display for AspectKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Radio(a, b) => write!(f, "{}/{}", a, b),
            Self::Global(g) => write!(f, "{}", g),
        }
    }
}

impl Display for TailwindAspect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "aspect-{}", self.kind)
    }
}

impl TailwindInstance for TailwindAspect {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "aspect-ratio" => self.kind
        }
    }
}
impl AspectKind {
    pub fn parse(kind: &[&str]) -> Result<Self> {
        let out = match kind {
            ["auto"] => Self::Auto,
            ["square"] => Self::Radio(1, 1),
            ["video"] => Self::Radio(16, 9),
            ["inherit"] => Self::Global(CssBehavior::Inherit),
            ["initial"] => Self::Global(CssBehavior::Initial),
            ["unset"] => Self::Global(CssBehavior::Unset),
            [n] => {
                let (a, b) = parse_fraction(n)?.1;
                Self::Radio(a, b)
            }
            _ => return syntax_error!("unknown aspect-ratio elements"),
        };
        Ok(out)
    }
}

impl TailwindAspect {
    /// https://tailwindcss.com/docs/aspect-ratio
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after aspect");
        Ok(Self { kind: AspectKind::parse(kind)? })
    }
}
