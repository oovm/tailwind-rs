use super::*;

impl Display for TailwindContainer {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindContainer {}

impl Display for BreakKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Before => f.write_str("break-before"),
            Self::After => f.write_str("break-after"),
            Self::Inside => f.write_str("break-inside"),
        }
    }
}

impl Display for TailwindBreakLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.kind, self.info)
    }
}

impl TailwindInstance for TailwindBreakLayout {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            self.kind => self.info
        }
    }
}

impl Display for BoxDecoration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clone => f.write_str("clone"),
            Self::Slice => f.write_str("slice"),
        }
    }
}

impl Display for TailwindBoxDecoration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "box-decoration-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBoxDecoration {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let decoration = self.kind.to_string();
        css_attributes! {
            "box-decoration-break" => decoration
        }
    }
}

impl Display for BoxSizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Border => f.write_str("border"),
            Self::Content => f.write_str("content"),
        }
    }
}

impl Display for TailwindBoxSizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "box-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBoxSizing {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let size = format!("{}-box", self.kind);
        css_attributes! {
            "box-sizing" => size
        }
    }
}

impl Display for TailwindDisplay {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindDisplay {}

impl Display for FloatKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => f.write_str("left"),
            Self::Right => f.write_str("right"),
            Self::None => f.write_str("none"),
        }
    }
}

impl Display for TailwindFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "float-{}", self.kind)
    }
}

impl TailwindInstance for TailwindFloat {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "float" => self.kind
        }
    }
}

impl Display for ClearKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => f.write_str("left"),
            Self::Right => f.write_str("right"),
            Self::Both => f.write_str("both"),
            Self::None => f.write_str("none"),
        }
    }
}

impl Display for TailwindClear {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "clear-{}", self.kind)
    }
}

impl TailwindInstance for TailwindClear {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "clear" => self.kind
        }
    }
}

impl Display for Isolation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Isolate => f.write_str("left"),
            Self::Auto => f.write_str("auto"),
        }
    }
}

impl Display for TailwindIsolation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            Isolation::Isolate => f.write_str("isolate"),
            Isolation::Auto => f.write_str("isolation-auto"),
        }
    }
}

impl TailwindInstance for TailwindIsolation {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "isolation" => self.kind
        }
    }
}

impl Display for ObjectFit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Contain => f.write_str("contain"),
            Self::Cover => f.write_str("cover"),
            Self::Fill => f.write_str("fill"),
            Self::ScaleDown => f.write_str("none"),
            Self::None => f.write_str("scale-down"),
        }
    }
}

impl Display for TailwindObjectFit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "object-{}", self.kind)
    }
}

impl TailwindInstance for TailwindObjectFit {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "object-fit" => self.kind
        }
    }
}

impl Display for AnchorPoint {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for AnchorPoint {}

impl Display for Overflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => f.write_str("auto"),
            Self::Hidden => f.write_str("hidden"),
            Self::Clip => f.write_str("clip"),
            Self::Visible => f.write_str("visible"),
            Self::Scroll => f.write_str("scroll"),
        }
    }
}

impl Display for TailwindOverflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            None => write!(f, "overflow-{}", self.kind),
            Some(true) => write!(f, "overflow-x-{}", self.kind),
            Some(false) => write!(f, "overflow-y-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindOverflow {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = match self.axis {
            None => "overflow",
            Some(true) => "overflow-x",
            Some(false) => "overflow-y",
        };
        css_attributes! {
            class => self
        }
    }
}

impl Display for Overscroll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => f.write_str("auto"),
            Self::Contain => f.write_str("contain"),
            Self::None => f.write_str("none"),
        }
    }
}

impl Display for TailwindOverscroll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            None => write!(f, "overscroll-{}", self.kind),
            Some(true) => write!(f, "overscroll-x-{}", self.kind),
            Some(false) => write!(f, "overscroll-y-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindOverscroll {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = match self.axis {
            None => "overscroll-behavior",
            Some(true) => "overscroll-behavior-x",
            Some(false) => "overscroll-behavior-y",
        };
        css_attributes! {
            class => self
        }
    }
}

impl Display for TailwindPosition {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}
