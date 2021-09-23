use super::*;

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
