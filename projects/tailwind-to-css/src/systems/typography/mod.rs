use crate::traits::CssAttribute;
use super::*;

#[doc = include_str ! ("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum FontSmoothing {
    Normal,
    Subpixel,
}

impl Display for FontSmoothing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(
                f,
                r#"\
-webkit-font-smoothing: antialiased;
-moz-osx-font-smoothing: grayscale;"#
            )?,
            Self::Subpixel => write!(
                f,
                r#"\
-webkit-font-smoothing: auto;
-moz-osx-font-smoothing: auto;"#
            )?,
        }
        Ok(())
    }
}


impl TailwindInstance for FontSmoothing {
    fn id(&self) -> String {
        match self {
            Self::Normal => "antialiased",
            Self::Subpixel => "subpixel-antialiased",
        }
            .to_string()
    }
    fn attributes(&self) -> Vec<CssAttribute> {
        match self {
            Self::Normal => vec![
                CssAttribute::new("-webkit-font-smoothing", "antialiased"),
                CssAttribute::new("-moz-osx-font-smoothing", "grayscale"),
            ],
            Self::Subpixel => vec![
                CssAttribute::new("-webkit-font-smoothing", "auto"),
                CssAttribute::new("-moz-osx-font-smoothing", "auto"),
            ],
        }
    }
}
