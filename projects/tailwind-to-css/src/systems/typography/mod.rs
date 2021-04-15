use std::fmt::{Debug, Display, Formatter};

#[doc = include_str ! ("font-smoothing.md")]
#[derive(Clone)]
pub enum FontSmoothing {
    Normal,
    Subpixel,
}

impl Display for FontSmoothing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => {
                write!(
                    f,
                    r#"\
-webkit-font-smoothing: antialiased;
-moz-osx-font-smoothing: grayscale;"#
                )
            }
            Self::Subpixel => {
                write!(
                    f,
                    r#"\
-webkit-font-smoothing: auto;
-moz-osx-font-smoothing: auto;"#
                )
            }
        }
        Ok(())
    }
}

impl Debug for FontSmoothing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
