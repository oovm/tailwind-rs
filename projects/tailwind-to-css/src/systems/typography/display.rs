use super::*;
use std::fmt::format;

impl Display for TailwindFontSmoothing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Normal => "antialiased",
            Self::Subpixel => "subpixel-antialiased",
        };
        f.write_str(text)
    }
}

impl TailwindInstance for TailwindFontSmoothing {
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

impl Display for TailwindFontFamily {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindFontFamily {}

impl Display for TailwindFontStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindFontStyle {}

impl Display for TailwindFontSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindFontSize {}

impl Display for TailwindTracking {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "tracking-[{}em]", self.em)
    }
}

impl TailwindInstance for TailwindTracking {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let em = format!("{}em", self.em);
        css_attributes! {
            "letter-spacing" => em
        }
    }
}

impl Display for TailwindLeading {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindLeading {}

impl Display for TailwindFontWeight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self.weight {
            100 => "thin",
            _ => return write!(f, "font-[{}]", self.weight),
        };
        write!(f, "font-{}", text)
    }
}

impl TailwindInstance for TailwindFontWeight {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        Iterator::collect(IntoIterator::into_iter([CssAttribute::new("font-weight", &self.weight.to_string())]))
    }
}

impl Display for TailwindTextAlignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindTextAlignment {}

impl Display for TailwindTextColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindTextColor {}
