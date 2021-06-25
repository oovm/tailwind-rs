use super::*;
use crate::parse_f32;
use tailwind_error::nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::opt, sequence::tuple, IResult,
};

impl TailwindFontArbitrary {
    pub fn parse() {}
}

impl TailwindFontFamily {
    #[inline]
    pub fn new(input: &str) -> Self {
        Self { name: input.to_string() }
    }
}

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
        match input {
            ["tighter"] => Ok(Self::Em(1.0)),
            ["tight"] => Ok(Self::Em(1.0)),
            // different from tailwind.js
            ["none"] => Ok(Self::Em(1.0)),
            ["wide"] => Ok(Self::Em(1.0)),
            ["wider" | "relaxed"] => Ok(Self::Em(1.0)),
            ["widest" | "loose"] => Ok(Self::Em(1.0)),
            ["normal"] => Ok(Self::Normal),
            [] => Self::parse_arbitrary(arbitrary),
            [n] => Self::parse_arbitrary(n),
            _ => syntax_error!("Unknown tracking instructions: {}", input.join("-")),
        }
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        Ok(Self::parse_em(arbitrary)?.1)
    }
    #[inline]
    fn parse_em(input: &str) -> IResult<&str, Self> {
        let (rest, (em, _)) = tuple((parse_integer, opt(tag("em"))))(input)?;
        Ok((rest, Self::Em(em)))
    }
}

impl TailwindLeading {
    pub fn parse(input: &[&str], arbitrary: &str) -> Result<Self> {
        match input {
            ["none"] => Ok(Self::Scale(1.0)),
            ["tight"] => Ok(Self::Scale(1.25)),
            ["snug"] => Ok(Self::Scale(1.375)),
            // different from tailwind.js
            ["wide"] => Ok(Self::Scale(1.5)),
            ["wider" | "relaxed"] => Ok(Self::Scale(1.625)),
            ["widest" | "loose"] => Ok(Self::Scale(2.0)),
            // https://developer.mozilla.org/zh-CN/docs/Web/CSS/line-height#normal
            ["normal"] => Ok(Self::Normal),
            [] => Self::parse_arbitrary(arbitrary),
            [n] => Self::parse_arbitrary(n),
            _ => syntax_error!("Unknown tracking instructions: {}", input.join("-")),
        }
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        let out = alt((Self::arbitrary_percent, Self::arbitrary_rem, Self::arbitrary_unit))(arbitrary)?;
        Ok(out.1)
    }
    #[inline]
    fn arbitrary_percent(input: &str) -> IResult<&str, Self> {
        let (rest, (f, _)) = tuple((parse_f32, char('%')))(input)?;
        Ok((rest, Self::Scale(f / 100.0)))
    }
    #[inline]
    fn arbitrary_unit(input: &str) -> IResult<&str, Self> {
        let (rest, u) = parse_integer(input)?;
        Ok((rest, Self::Unit(u)))
    }
    // #[inline]
    // fn arbitrary_px(input: &str) -> IResult<&str, Self> {
    //     let (rest, (f, _)) = tuple((parse_f32, tag("px")))(input)?;
    //     Ok((rest, Self::Px(f)))
    // }
    #[inline]
    fn arbitrary_rem(input: &str) -> IResult<&str, Self> {
        let (rest, (f, _)) = tuple((parse_f32, tag("rem")))(input)?;
        Ok((rest, Self::Rem(f)))
    }
}

impl TailwindListStyle {
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        Ok(Self::Custom(arbitrary.to_string()))
    }
}

impl TailwindFontSize {
    #[inline]
    pub fn new(size: f32, height: f32) -> Self {
        Self { size: TailwindTracking::Em(size), height: TailwindLeading::Rem(height) }
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

impl TailwindUnderlineOffset {
    pub fn parse(input: &[&str], arbitrary: &str) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        todo!()
    }
}
