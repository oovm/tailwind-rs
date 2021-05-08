use super::*;

#[doc = include_str ! ("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum FontSmoothing {
    Normal,
    Subpixel,
}

impl TailwindInstance for FontSmoothing {
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

impl FontSmoothing {
    pub fn parse_antialias(input: &str) -> ParsedItem {
        match tag("antialiased")(input) {
            Ok(o) => Ok((o.0, Box::new(FontSmoothing::Normal))),
            Err(e) => Err(e),
        }
    }

    pub fn parse_subpixel(input: &str) -> ParsedItem {
        match tag("subpixel-antialiased")(input) {
            Ok(o) => Ok((o.0, Box::new(FontSmoothing::Subpixel))),
            Err(e) => Err(e),
        }
    }
}
