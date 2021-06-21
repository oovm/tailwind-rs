use super::*;
use tailwind_error::nom::{bytes::complete::tag, combinator::opt, sequence::tuple};

impl TailwindFontSmoothing {
    #[inline]
    pub fn new(subpixel: bool) -> Self {
        match subpixel {
            true => Self::Subpixel,
            false => Self::Normal,
        }
    }
}

impl TailwindTracking {
    pub fn parse(input: &[&str], arbitrary: &str) -> Result<Self> {
        let em = match input {
            ["tighter"] => -0.05,
            ["tight"] => -0.25,
            ["normal"] => 0.0,
            ["wide"] => 0.025,
            ["wider"] => 0.05,
            ["widest"] => 0.1,
            [] => return Self::parse_arbitrary(arbitrary),
            [n] => return Self::parse_arbitrary(n),
            _ => return syntax_error!("Unknown tracking instructions: {}", input.join("-")),
        };
        Ok(Self { em })
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        let (em, _) = tuple((parse_integer, opt(tag("em"))))(arbitrary)?.1;
        Ok(Self { em })
    }
}

impl TailwindFontSize {
    #[inline]
    pub fn new(size: f32, height: f32) -> Self {
        Self { size: size, height: height }
    }
}

impl TailwindFontWeight {
    pub const THIN: Self = Self { weight: 100 };
    pub const EXTRA_LIGHT: Self = Self { weight: 200 };
    pub const LIGHT: Self = Self { weight: 300 };
    pub const NORMAL: Self = Self { weight: 400 };
    pub const MEDIUM: Self = Self { weight: 500 };
    pub const SEMI_BOLD: Self = Self { weight: 600 };
    pub const BOLD: Self = Self { weight: 700 };
    pub const EXTRA_BOLD: Self = Self { weight: 800 };
    pub const BLACK: Self = Self { weight: 900 };
    #[inline]
    pub fn new(weight: usize) -> Self {
        Self { weight }
    }
}
