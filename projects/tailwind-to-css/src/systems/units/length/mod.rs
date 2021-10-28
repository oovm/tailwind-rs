use tailwind_ast::parse_fraction;

use super::*;

#[derive(Debug, Copy, Clone)]
pub enum LengthUnit {
    Fraction(usize, usize),
    Unit(f32, &'static str),
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fraction(a, b) => write!(f, "{}/{}", a, b),
            Self::Unit(a, b) => write!(f, "{}{}", *a as usize, b),
        }
    }
}

impl LengthUnit {
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/length#syntax
    pub fn parse_faction(input: &str) -> Result<Self> {
        let (a, b) = parse_fraction(input)?.1;
        Ok(Self::Fraction(a, b))
    }
    pub fn parse_length(input: &str) -> Result<Self> {
        let valid = (unit("px"), unit("em"), unit("rem"), unit("%"));
        let (f, unit) = tuple((parse_f32, alt(valid)))(input)?.1;
        Ok(Self::Unit(f, unit))
    }
    pub fn parse_angle(input: &str) -> Result<Self> {
        let valid = (unit("deg"), unit("rad"));
        let (f, unit) = tuple((parse_f32, alt(valid)))(input)?.1;
        Ok(Self::Unit(f, unit))
    }
    pub fn px(x: f32) -> Self {
        Self::Unit(x, "px")
    }
    pub fn em(x: f32) -> Self {
        Self::Unit(x, "em")
    }
    pub fn rem(x: f32) -> Self {
        Self::Unit(x, "rem")
    }
    pub fn percent(x: f32) -> Self {
        Self::Unit(x, "%")
    }
}

fn unit(unit: &'static str) -> impl Fn(&str) -> IResult<&str, &'static str> {
    move |input: &str| tag(unit)(input).map(|(s, _)| (s, unit))
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
            Self::Fraction(a, b) => {
                let p = *a as f32 / *b as f32;
                format!("{}%", 100.0 * p)
            },
            Self::Unit(a, b) => format!("{}{}", a, b),
        }
    }

    pub fn is_fraction(&self) -> bool {
        matches!(self, Self::Fraction { .. })
    }
}
