use super::*;

#[derive(Clone, Debug)]
pub enum AnchorPoint {
    LeftTop,
    Top,
    RightTop,
    Left,
    Center,
    Right,
    LeftBottom,
    Bottom,
    RightBottom,
    Custom(String),
    Global(CssBehavior),
}

pub struct TailwindOrigin {
    kind: AnchorPoint,
}

// transform-origin: inherit;
// transform-origin: initial;
// transform-origin: unset;
impl AnchorPoint {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let out = match pattern {
            ["7"] | ["left", "top"] | ["top", "left"] => Self::LeftTop,
            ["8"] | ["top"] => Self::Top,
            ["9"] | ["right", "top"] | ["top", "right"] => Self::RightTop,
            ["4"] | ["left"] => Self::Left,
            ["5"] | ["center"] => Self::Center,
            ["6"] | ["right"] => Self::Right,
            ["1"] | ["left", "bottom"] | ["bottom", "left"] => Self::LeftBottom,
            ["2"] | ["bottom"] => Self::Bottom,
            ["3"] | ["right", "bottom"] | ["bottom", "right"] => Self::RightBottom,
            [] => Self::parse_arbitrary(arbitrary)?,
            _ => syntax_error!("Unknown object instructions: {}", pattern.join("-"))?,
        };
        Ok(out)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Custom(arbitrary.to_string()))
    }

    pub fn get_class(&self) -> String {
        match self {
            Self::Global(g) => g.to_string(),
            _ => format!("[{}]", self.get_properties()),
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
            Self::Custom(c) => c.to_string(),
            Self::Global(g) => g.to_string(),
        }
    }
}
