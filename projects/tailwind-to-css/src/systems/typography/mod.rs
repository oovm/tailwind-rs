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
    fn attributes(&self) -> Vec<String> {
        match self {
            Self::Normal => vec![
                //
                "-webkit-font-smoothing: antialiased;".to_string(),
                "-moz-osx-font-smoothing: grayscale;".to_string(),
            ],
            Self::Subpixel => vec![
                //
                "-webkit-font-smoothing: auto;".to_string(),
                "-moz-osx-font-smoothing: auto;".to_string(),
            ],
        }
    }
}
