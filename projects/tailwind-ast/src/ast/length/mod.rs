use std::ops::Rem;

use super::*;

#[derive(Debug, Copy, Clone)]
pub enum LengthUnit {
    Fraction(u32, u32),
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
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/length#syntax>
    pub fn parse_faction(input: &str) -> Result<Self> {
        let (a, b) = parse_fraction(input)?.1;
        Ok(Self::radio(a as u32, b as u32))
    }
    pub fn parse_length(input: &str) -> Result<Self> {
        let valid = (unit("px"), unit("em"), unit("rem"), unit("%"));
        let (f, unit) = tuple((parse_f32, alt(valid)))(input)?.1;
        Ok(Self::Unit(f, unit))
    }
    pub fn parse_angle(input: &str) -> Result<Self> {
        let valid = (unit("deg"), unit("rad"), unit("grad"), unit("turn"));
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
    pub fn radio(a: u32, b: u32) -> Self {
        if b.eq(&0) {
            return Self::Fraction(0, 1);
        }
        let n = gcd(a, b);
        Self::Fraction(a / n, b / n)
    }
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: PartialEq + Rem<Output = T> + Default + Copy,
{
    if b == T::default() { a } else { gcd(b, a % b) }
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
    pub fn is_fraction_eq(&self) -> bool {
        match self {
            Self::Fraction(a, b) => a.eq(b),
            _ => false,
        }
    }
    pub fn is_fraction_zero(&self) -> bool {
        match self {
            Self::Fraction(a, _) => a.eq(&0),
            _ => false,
        }
    }
}
