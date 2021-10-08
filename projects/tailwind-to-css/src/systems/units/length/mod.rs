use super::*;

#[derive(Debug, Copy, Clone)]
pub enum LengthUnit {
    Px(f32),
    Em(f32),
    Rem(f32),
    Percent(f32),
    Fraction(usize, usize),
    Degree(f32),
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Px(n) => write!(f, "{}px", *n as usize),
            Self::Em(n) => write!(f, "{}em", *n as usize),
            Self::Rem(n) => write!(f, "{}rem", *n as usize),
            Self::Percent(n) => write!(f, "{}%", *n as usize),
            Self::Fraction(a, b) => write!(f, "{}/{}", a, b),
            Self::Degree(n) => write!(f, "{}deg", *n as usize),
        }
    }
}

impl LengthUnit {
    pub fn fraction(numerator: usize, denominator: usize) -> Self {
        Self::Fraction(numerator, denominator)
    }

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

impl LengthUnit {
    #[inline]
    pub fn get_class(&self) -> String {
        self.to_string()
    }
    #[inline]
    pub fn get_class_arbitrary(&self) -> String {
        format!("[{}]", self)
    }
    #[inline]
    pub fn get_properties(&self) -> String {
        match self {
            Self::Px(n) => format!("{}px", n),
            Self::Em(n) => format!("{}em", n),
            Self::Rem(n) => format!("{}rem", n),
            Self::Percent(n) => format!("{}%", n),
            Self::Fraction(a, b) => format!("{}/{}", a, b),
            Self::Degree(n) => format!("{}deg", n),
        }
    }

    pub fn is_fraction(&self) -> bool {
        matches!(self, Self::Fraction { .. })
    }
}
