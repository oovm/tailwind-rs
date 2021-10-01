use super::*;

#[derive(Copy, Clone, Debug)]
enum Overflow {
    Auto,
    Hidden,
    Clip,
    Visible,
    Scroll,
}

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindOverflow {
    kind: Overflow,
    axis: Option<bool>,
}

impl Display for Overflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => f.write_str("auto"),
            Self::Hidden => f.write_str("hidden"),
            Self::Clip => f.write_str("clip"),
            Self::Visible => f.write_str("visible"),
            Self::Scroll => f.write_str("scroll"),
        }
    }
}

impl Display for TailwindOverflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            None => write!(f, "overflow-{}", self.kind),
            Some(true) => write!(f, "overflow-x-{}", self.kind),
            Some(false) => write!(f, "overflow-y-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindOverflow {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = match self.axis {
            None => "overflow",
            Some(true) => "overflow-x",
            Some(false) => "overflow-y",
        };
        css_attributes! {
            class => self
        }
    }
}

impl Overflow {
    pub fn parse(input: &[&str]) -> Result<Self> {
        match input {
            ["auto"] => Ok(Self::Auto),
            ["hidden"] => Ok(Self::Hidden),
            ["clip"] => Ok(Self::Clip),
            ["visible"] => Ok(Self::Visible),
            ["scroll"] => Ok(Self::Scroll),
            _ => syntax_error!("Unknown overflow instructions: {}", input.join("-")),
        }
    }
}

impl TailwindOverflow {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary, axis: Option<bool>) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after overflow");
        let kind = Overflow::parse(kind)?;
        Ok(Self { kind, axis })
    }
}
