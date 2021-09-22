use super::*;

impl Display for TailwindFontArbitrary {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindFontArbitrary {}

impl Display for TailwindFontSize {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl Display for FontVariantNumeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => f.write_str("normal"),
            Self::Ordinal => f.write_str("ordinal"),
            Self::SlashedZero => f.write_str("slashed-zero"),
            Self::Lining => f.write_str("lining-nums"),
            Self::OldStyle => f.write_str("oldstyle-nums"),
            Self::Proportional => f.write_str("proportional-nums"),
            Self::Tabular => f.write_str("tabular-nums"),
            Self::DiagonalFractions => f.write_str("diagonal-fractions"),
            Self::StackedFractions => f.write_str("stacked-fractions"),
        }
    }
}

impl Display for TailwindFontVariantNumeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            FontVariantNumeric::Normal => write!(f, "normal-nums"),
            _ => write!(f, "{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindFontVariantNumeric {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "font-variant-numeric" => self.kind
        }
    }
}

impl Display for Tracking {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Global(g) => write!(f, "{}", g),
            Self::Length(l) => write!(f, "[{}]", l),
        }
    }
}

impl Display for TailwindTracking {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "tracking-{}", self.kind)
    }
}

impl TailwindInstance for TailwindTracking {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let spacing = match self.kind {
            Tracking::Length(n) => format!("{}", n),
            _ => self.kind.to_string(),
        };
        css_attributes! {
            "letter-spacing" => spacing
        }
    }
}

impl Display for Leading {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Length(n) => write!(f, "[{}]", n),
            Self::Global(g) => write!(f, "{}", g),
        }
    }
}

impl Display for TailwindLeading {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "leading-{}", self.kind)
    }
}

impl TailwindInstance for TailwindLeading {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let leading = match self.kind {
            Leading::Normal => "normal".to_string(),
            Leading::Length(n) => n.to_string(),
            Leading::Global(g) => g.to_string(),
        };
        css_attributes! {
            "line-height" => leading
        }
    }
}

impl Display for TailwindListStyle {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindListStyle {}

impl Display for TailwindListStylePosition {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindListStylePosition {}

impl Display for TailwindTextAlignment {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindTextAlignment {}

impl Display for TailwindTextColor {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindTextColor {}

impl Display for TailwindUnderlineOffset {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindUnderlineOffset {}

impl Display for TailwindTextOverflow {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
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
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindIndent {}

impl Display for TailwindAlign {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindAlign {}

impl Display for TailwindWhiteSpace {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindWhiteSpace {}

impl Display for TailwindBreak {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindBreak {}

impl Display for TailwindContentElement {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindContentElement {}
