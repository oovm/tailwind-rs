use super::*;

#[derive(Clone, Debug)]
enum Transition {
    None,
    All,
    Default,
    Colors,
    Opacity,
    Shadow,
    Transform,
    Arbitrary(String),
}

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindTransition {
    kind: Transition,
}

impl Display for Transition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "-none"),
            Self::All => write!(f, "-all"),
            Self::Default => write!(f, ""),
            Self::Colors => write!(f, "-colors"),
            Self::Opacity => write!(f, "-opacity"),
            Self::Shadow => write!(f, "-shadow"),
            Self::Transform => write!(f, "-transform"),
            Self::Arbitrary(g) => write!(f, "-[{}]", g),
        }
    }
}

impl Display for TailwindTransition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "transition[{}]", self.kind)
    }
}

impl TailwindInstance for TailwindTransition {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {}
    }
}

impl TailwindTransition {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Transition::parse(pattern, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Transition::parse_arbitrary(arbitrary)? })
    }
}

impl Transition {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let t = match pattern {
            [] if arbitrary.is_none() => Self::Default,
            [] => Self::parse_arbitrary(arbitrary)?,
            ["none"] => Self::None,
            ["all"] => Self::All,
            ["colors"] => Self::Colors,
            ["opacity"] => Self::Opacity,
            ["shadow"] => Self::Shadow,
            ["transform"] => Self::Transform,
            _ => return syntax_error!("Unknown transition instructions: {}", pattern.join("-")),
        };
        Ok(t)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self::Arbitrary(arbitrary.to_string()))
    }
}
