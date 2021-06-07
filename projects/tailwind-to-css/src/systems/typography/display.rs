use super::*;

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

impl TailwindInstance for TailwindFontFamily {
    fn id(&self) -> String {
        todo!()
    }
}

impl TailwindInstance for TailwindFontSize {
    fn id(&self) -> String {
        todo!()
    }
}

impl TailwindInstance for TailwindFontWeight {
    fn id(&self) -> String {
        let text = match self.weight {
            100 => "thin",
            _ => return format!("font-[{}]", self.weight),
        };
        format!("font-{}", text)
    }
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        Iterator::collect(IntoIterator::into_iter([CssAttribute::new("font-weight", &self.weight.to_string())]))
    }
}
