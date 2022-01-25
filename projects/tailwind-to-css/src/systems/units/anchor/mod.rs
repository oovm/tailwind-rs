use super::*;

/// Anchor points are used to position elements relative to the viewport.
#[derive(Clone, Debug)]
pub enum AnchorPoint {
    /// `0% 0%` to the viewport.
    LeftTop,
    /// `50% 0%` to the viewport.
    Top,
    /// `100% 0%` to the viewport.
    RightTop,
    /// `0% 50%` to the viewport.
    Left,
    /// `50% 50%` to the viewport.
    Center,
    /// `100% 50%` to the viewport.
    Right,
    /// `0% 100%` to the viewport.
    LeftBottom,
    /// `50% 100%` to the viewport.
    Bottom,
    /// `100% 100%` to the viewport.
    RightBottom,
    Standard(String),
    Arbitrary(TailwindArbitrary),
}

impl AnchorPoint {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, allow_center: bool) -> Result<Self> {
        let out = match pattern {
            ["7" | "tl" | "lt"] | ["left", "top"] | ["top", "left"] => Self::LeftTop,
            ["8" | "t"] | ["top"] => Self::Top,
            ["9" | "rt" | "tr"] | ["right", "top"] | ["top", "right"] => Self::RightTop,
            ["4" | "l"] | ["left"] => Self::Left,
            ["5" | "c"] | ["center"] if allow_center => Self::Center,
            ["6" | "r"] | ["right"] => Self::Right,
            ["1" | "lb" | "bl"] | ["left", "bottom"] | ["bottom", "left"] => Self::LeftBottom,
            ["2" | "b"] | ["bottom"] => Self::Bottom,
            ["3" | "rb" | "br"] | ["right", "bottom"] | ["bottom", "right"] => Self::RightBottom,
            [n] if Self::check_valid(n) => Self::Standard(n.to_string()),
            [] => Self::parse_arbitrary(arbitrary)?,
            _ => return syntax_error!("Unknown anchor-point instructions: {}", pattern.join("-")),
        };
        Ok(out)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }

    pub fn get_class(&self) -> String {
        match self {
            Self::LeftTop => "7".to_string(),
            Self::Top => "8".to_string(),
            Self::RightTop => "9".to_string(),
            Self::Left => "4".to_string(),
            Self::Center => "5".to_string(),
            Self::Right => "6".to_string(),
            Self::LeftBottom => "1".to_string(),
            Self::Bottom => "2".to_string(),
            Self::RightBottom => "3".to_string(),
            Self::Standard(s) => s.to_string(),
            Self::Arbitrary(s) => s.get_class(),
        }
    }

    pub fn get_properties(&self) -> String {
        match self {
            Self::LeftTop => "0% 0%".to_string(),
            Self::Top => "50% 0%".to_string(),
            Self::RightTop => "100% 0%".to_string(),
            Self::Left => "0% 50%".to_string(),
            Self::Center => "50% 50%".to_string(),
            Self::Right => "100% 50%".to_string(),
            Self::LeftBottom => "0% 100%".to_string(),
            Self::Bottom => "50% 100%".to_string(),
            Self::RightBottom => "100% 100%".to_string(),
            Self::Standard(s) => s.to_string(),
            Self::Arbitrary(s) => s.get_properties(),
        }
    }
    pub fn check_valid(mode: &str) -> bool {
        ["inherit", "initial", "revert", "unset"].contains(&mode)
    }
}
