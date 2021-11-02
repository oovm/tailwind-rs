use super::*;

#[derive(Copy, Clone, Debug)]
pub(super) enum PaddingAxis {
    Padding,
    PaddingL,
    PaddingR,
    PaddingT,
    PaddingB,
    PaddingX,
    PaddingY,
}

impl Display for PaddingAxis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Padding => write!(f, "p"),
            Self::PaddingL => write!(f, "pl"),
            Self::PaddingR => write!(f, "pr"),
            Self::PaddingT => write!(f, "pt"),
            Self::PaddingB => write!(f, "pb"),
            Self::PaddingX => write!(f, "px"),
            Self::PaddingY => write!(f, "py"),
        }
    }
}

impl PaddingAxis {
    pub fn parse_axis(axis: &str) -> Self {
        match axis {
            "p" => Self::Padding,
            "px" => Self::PaddingX,
            "py" => Self::PaddingY,
            "pl" => Self::PaddingL,
            "pr" => Self::PaddingR,
            "pt" => Self::PaddingT,
            "pb" => Self::PaddingB,
            _ => unreachable!(),
        }
    }
    pub fn get_attributes(&self, value: String) -> BTreeSet<CssAttribute> {
        match self {
            Self::Padding => css_attributes! {
                "padding" => &value
            },
            Self::PaddingL => css_attributes! {
                "padding-left" => &value
            },
            Self::PaddingR => css_attributes! {
                "padding-right" => &value
            },
            Self::PaddingT => css_attributes! {
                "padding-top" => &value
            },
            Self::PaddingB => css_attributes! {
                "padding-bottom" => &value
            },
            Self::PaddingX => css_attributes! {
                "padding-left" => &value,
                "padding-right" => &value
            },
            Self::PaddingY => css_attributes! {
                "padding-top" => &value,
                "padding-bottom" => &value
            },
        }
    }
}
