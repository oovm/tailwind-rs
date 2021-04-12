use core::num::flt2dec::decode;
use std::fmt::{Debug, Display, Formatter, Write};
use css_style::unit::{Length, Percent, px, Rem};

pub struct SizingSystem {}

/// ## `w`
/// https://www.tailwindcss.cn/docs/width
pub enum TailwindWidth {
    Min,
    Max,
    Fit,
    Full,
    Auto,
    Screen,
    Length(Length)
}


impl TailwindWidth {
    /// `w-px`
    pub fn px(n: usize) -> Self {
        Self::Length(px(n))
    }
    /// `w-{number}`
    pub fn number(number: usize) -> Self {
        Self::Length(Length::Rem(Rem(number as f32 / 4.0)))
    }
    /// `w-[{n}rem]`
    pub fn rem(number: usize) -> Self {
        Self::Length(Length::Rem(Rem(number as f32 / 4.0)))
    }
    /// `w-{a}/{b}` & `w-full`
    pub fn percent(numerator: usize, denominator: usize) -> Self {
        assert!(numerator <= denominator);
        if numerator == denominator {
            Self::Full
        } else {
            Self::Percent(Percent(numerator as f32 / denominator as f32))
        }
    }
}

impl Debug for TailwindWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("width: ")?;
        match self {
            Self::Min => { f.write_str("min-content")? }
            Self::Max => { f.write_str("max-content")? }
            Self::Fit => { f.write_str("fit-content")? }
            Self::Screen => { f.write_str("fit-content")? }
            Self::Full => { f.write_str("fit-content")? }
            Self::Auto => { f.write_str("auto")? }
            Self::Length(n) => {Debug::fmt(n, f)}
        }
        f.write_char(';')
    }
}


#[test]
fn width() {
    println!("{:#?}", TailwindWidth::percent(1, 2))
}