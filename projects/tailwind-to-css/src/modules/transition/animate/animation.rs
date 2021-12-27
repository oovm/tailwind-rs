use super::*;

#[derive(Clone, Debug)]
pub(super) enum Animation {
    None,
    Spin,
    Ping,
    Pulse,
    Bounce,
    Arbitrary(TailwindArbitrary),
}

impl Display for Animation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Spin => write!(f, "spin"),
            Self::Ping => write!(f, "ping"),
            Self::Pulse => write!(f, "pulse"),
            Self::Bounce => write!(f, "bounce"),
            Self::Arbitrary(s) => s.write(f),
        }
    }
}

impl Animation {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["none"] => Self::None,
            ["spin"] => Self::Spin,
            ["ping"] => Self::Ping,
            ["pulse"] => Self::Pulse,
            ["bounce"] => Self::Bounce,
            [] => Self::parse_arbitrary(arbitrary)?,
            _ => return syntax_error!("Unknown outline instructions: {}", pattern.join("-")),
        };
        Ok(kind)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
}
