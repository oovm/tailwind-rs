use super::*;

impl Display for TailwindFontFamily {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindFontFamily {}

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
        f.write_str("tracking-")?;
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Inherit => write!(f, "inherit"),
            Self::Initial => write!(f, "initial"),
            Self::Unset => write!(f, "unset"),
            Self::Em(em) => write!(f, "[{}em]", em),
        }
    }
}

impl TailwindInstance for TailwindTracking {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let em = match self {
            Self::Normal => format!("normal"),
            Self::Inherit => format!("inherit"),
            Self::Initial => format!("initial"),
            Self::Unset => format!("unset"),
            Self::Em(em) => format!("{}em", em),
        };
        css_attributes! {
            "letter-spacing" => em
        }
    }
}

impl Display for TailwindLeading {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("leading-")?;
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Inherit => write!(f, "inherit"),
            Self::Initial => write!(f, "initial"),
            Self::Unset => write!(f, "unset"),
            Self::Unit(n) => write!(f, "{}", n),
            Self::Scale(n) => write!(f, "[{}%]", n * 100.0),
            Self::Rem(n) => write!(f, "[{}rem]", n),
            // Self::Px(n) => write!(f, "[{}px]", n),
        }
    }
}

impl TailwindInstance for TailwindLeading {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let leading = match self {
            TailwindLeading::Normal => {}
            TailwindLeading::Inherit => {}
            TailwindLeading::Initial => {}
            TailwindLeading::Unset => {}
            TailwindLeading::Unit(_) => {}
            TailwindLeading::Scale(_) => {}
            TailwindLeading::Rem(_) => {} // TailwindLeading::Px(_) => {}
        };
        css_attributes! {
            "line-height" => ""
        }
    }
}

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

impl Display for TailwindUnderlineOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindUnderlineOffset {}
