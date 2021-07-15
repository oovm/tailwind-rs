use super::*;

impl Display for TailwindFontArbitrary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindFontArbitrary {}

impl Display for TailwindFontSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for TailwindFontFamily {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "font-{}", self.name)
    }
}

impl TailwindInstance for TailwindFontFamily {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let family = ctx.fonts.get_family(&self.name);
        css_attributes! {
            "font-family" => family
        }
    }
}

impl TailwindInstance for TailwindFontSize {}

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

// Class
// Properties
// normal-nums	font-variant-numeric: normal;
// ordinal	font-variant-numeric: ordinal;
// slashed-zero	font-variant-numeric: slashed-zero;
// lining-nums	font-variant-numeric: lining-nums;
// oldstyle-nums	font-variant-numeric: oldstyle-nums;
// proportional-nums	font-variant-numeric: proportional-nums;
// tabular-nums	font-variant-numeric: tabular-nums;
// diagonal-fractions	font-variant-numeric: diagonal-fractions;
// stacked-fractions	font-variant-numeric: stacked-fractions;
impl Display for TailwindFontVariantNumeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindFontVariantNumeric {}

impl Display for TailwindTracking {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("tracking-")?;
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Global(g) => write!(f, "{}", g),
            Self::Em(em) => write!(f, "[{}em]", em),
        }
    }
}

impl TailwindInstance for TailwindTracking {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let em = match self {
            Self::Normal => format!("normal"),
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

impl Display for TailwindListStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindListStyle {}

impl Display for TailwindListStylePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindListStylePosition {}

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

impl Display for TailwindTextDecoration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindTextDecoration {}

impl Display for TailwindDecorationColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindDecorationColor {}

impl Display for TailwindDecorationStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindDecorationStyle {}

impl Display for TailwindDecorationThickness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindDecorationThickness {}

impl Display for TailwindUnderlineOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindUnderlineOffset {}

impl Display for TailwindTextTransform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// Class
// Properties
// uppercase	text-transform: uppercase;
// lowercase	text-transform: lowercase;
// capitalize	text-transform: capitalize;
// normal-case	text-transform: none;
impl TailwindInstance for TailwindTextTransform {}

impl Display for TailwindTextOverflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// Class
// Properties
// truncate	overflow: hidden;
// text-overflow: ellipsis;
// white-space: nowrap;
// text-ellipsis	text-overflow: ellipsis;
// text-clip	text-overflow: clip;
impl TailwindInstance for TailwindTextOverflow {}

impl Display for TailwindIndent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindIndent {}

impl Display for TailwindAlign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindAlign {}

impl Display for TailwindWhiteSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindWhiteSpace {}

impl Display for TailwindBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindBreak {}

impl Display for TailwindContentElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindContentElement {}
