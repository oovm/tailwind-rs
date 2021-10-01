use super::*;

#[derive(Copy, Clone, Debug)]
enum Overscroll {
    Auto,
    Contain,
    None,
}

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindOverscroll {
    kind: Overscroll,
    axis: Option<bool>,
}

impl Display for Overscroll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => f.write_str("auto"),
            Self::Contain => f.write_str("contain"),
            Self::None => f.write_str("none"),
        }
    }
}

impl Display for TailwindOverscroll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            None => write!(f, "overscroll-{}", self.kind),
            Some(true) => write!(f, "overscroll-x-{}", self.kind),
            Some(false) => write!(f, "overscroll-y-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindOverscroll {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = match self.axis {
            None => "overscroll-behavior",
            Some(true) => "overscroll-behavior-x",
            Some(false) => "overscroll-behavior-y",
        };
        css_attributes! {
            class => self
        }
    }
}

impl Overscroll {
    #[inline]
    pub fn parse(input: &[&str]) -> Result<Self> {
        match input {
            ["auto"] => Ok(Self::Auto),
            ["contain"] => Ok(Self::Contain),
            ["none"] => Ok(Self::None),
            _ => syntax_error!("Unknown overflow instructions: {}", input.join("-")),
        }
    }
}

impl TailwindOverscroll {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary, axis: Option<bool>) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after overflow");
        let kind = Overscroll::parse(kind)?;
        Ok(Self { kind, axis })
    }
}
