use super::*;

#[doc = include_str ! ("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindFontSmoothing {
    Normal,
    Subpixel,
}

impl TailwindInstance for TailwindFontSmoothing {
    fn id(&self) -> String {
        match self {
            Self::Normal => "antialiased",
            Self::Subpixel => "subpixel-antialiased",
        }
        .to_string()
    }
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        match self {
            Self::Normal => css_attributes! {
                "-webkit-font-smoothing" => "antialiased",
                "-moz-osx-font-smoothing" => "grayscale",
            },
            Self::Subpixel => css_attributes! {
                "-webkit-font-smoothing" => "auto",
                "-moz-osx-font-smoothing" => "auto",
            },
        }
    }
}

impl TailwindFontSmoothing {
    pub fn parse(subpixel: bool) -> Box<dyn TailwindInstance> {
        let out = match subpixel {
            true => Self::Subpixel,
            false => Self::Normal,
        };
        Box::new(out)
    }
}
