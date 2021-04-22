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

impl CssInstance for FontSmoothing {
    fn selectors(&self) -> &'static str {
        match self {
            Self::Normal => "antialiased",
            Self::Subpixel => "subpixel-antialiased",
        }
    }
    fn attributes(&self) -> Vec<&'static str> {
        match self {
            Self::Normal => {
                vec!["-webkit-font-smoothing: antialiased;", "-moz-osx-font-smoothing: grayscale;"]
            }
            Self::Subpixel => {
                vec!["-webkit-font-smoothing: auto;", "-moz-osx-font-smoothing: auto;"]
            }
        }
    }
}
