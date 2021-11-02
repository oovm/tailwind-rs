use super::*;

#[derive(Copy, Clone, Debug)]
pub(super) enum MarginAxis {
    Margin,
    MarginL,
    MarginR,
    MarginT,
    MarginB,
    MarginX,
    MarginY,
}

impl Display for MarginAxis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Margin => write!(f, "m"),
            Self::MarginL => write!(f, "ml"),
            Self::MarginR => write!(f, "mr"),
            Self::MarginT => write!(f, "mt"),
            Self::MarginB => write!(f, "mb"),
            Self::MarginX => write!(f, "mx"),
            Self::MarginY => write!(f, "my"),
        }
    }
}

impl MarginAxis {
    pub fn parse_axis(axis: &str) -> Self {
        match axis {
            "m" => Self::Margin,
            "mx" => Self::MarginX,
            "my" => Self::MarginY,
            "ml" => Self::MarginL,
            "mr" => Self::MarginR,
            "mt" => Self::MarginT,
            "mb" => Self::MarginB,
            _ => unreachable!(),
        }
    }
    pub fn get_attributes(&self, value: String) -> BTreeSet<CssAttribute> {
        match self {
            Self::Margin => css_attributes! {
                "margin" => &value
            },
            Self::MarginL => css_attributes! {
                "margin-left" => &value
            },
            Self::MarginR => css_attributes! {
                "margin-right" => &value
            },
            Self::MarginT => css_attributes! {
                "margin-top" => &value
            },
            Self::MarginB => css_attributes! {
                "margin-bottom" => &value
            },
            Self::MarginX => css_attributes! {
                "margin-left" => &value,
                "margin-right" => &value
            },
            Self::MarginY => css_attributes! {
                "margin-top" => &value,
                "margin-bottom" => &value
            },
        }
    }
}
