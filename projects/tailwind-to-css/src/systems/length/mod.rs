use super::*;

#[derive(Copy, Clone)]
pub enum LengthUnit {
    Px(f32),
    Em(f32),
    Rem(f32),
    Percent(f32),
}

impl Debug for LengthUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Px(n) => write!(f, "{}px", n),
            Self::Em(n) => write!(f, "{}em", n),
            Self::Rem(n) => write!(f, "{}rem", n),
            Self::Percent(n) => write!(f, "{}%", n),
        }
    }
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Px(n) => write!(f, "{}px", *n as usize),
            Self::Em(n) => write!(f, "{}em", *n as usize),
            Self::Rem(n) => write!(f, "{}rem", *n as usize),
            Self::Percent(n) => write!(f, "{}%", *n as usize),
        }
    }
}

impl LengthUnit {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, (f, unit)) = tuple((parse_f32, Self::parse_unit))(input)?;
        let out = match unit {
            "px" => Self::Px(f),
            "em" => Self::Em(f),
            "rem" => Self::Rem(f),
            "%" => Self::Percent(f),
            _ => return Err(Failure(Error::new("Unsolved unit", ErrorKind::Alpha))),
        };
        Ok((rest, out))
    }

    fn parse_unit(input: &str) -> IResult<&str, &str> {
        alt((tag("px"), tag("em"), tag("rem"), tag("%")))(input)
    }
}
