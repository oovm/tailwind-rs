use super::*;

impl Display for AspectKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Radio(a, b) => write!(f, "{}/{}", a, b),
            Self::Global(g) => write!(f, "{}", g),
        }
    }
}

impl Display for TailwindAspect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "aspect-{}", self.kind)
    }
}

impl TailwindInstance for TailwindAspect {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "aspect-ratio" => self.kind
        }
    }
}

impl Display for TailwindContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
        let class = self.kind.to_string();
        let breaking = self.info.to_string();
        css_attributes! {
            class => breaking
        }
    }
}

impl Display for ColumnKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Columns(n) => write!(f, "{}", n),
            Self::Length(n) => write!(f, "{}", n),
        }
    }
}

impl Display for TailwindColumns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "columns-{}", self.kind)
    }
}

impl TailwindInstance for TailwindColumns {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let columns = match self.kind {
            ColumnKind::Auto => "auto".to_string(),
            ColumnKind::Columns(n) => format!("{}", n),
            ColumnKind::Length(n) => format!("{:?}", n),
        };
        css_attributes! {
            "columns" => columns
        }
    }
}

// Class
// Properties
// box-decoration-clone	box-decoration-break: clone;
// box-decoration-slice	box-decoration-break: slice;
impl Display for TailwindBoxDecoration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindBoxDecoration {
    fn id(&self) -> String {
        todo!()
    }
}

impl Display for TailwindBoxSizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindBoxSizing {}

impl Display for TailwindDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindDisplay {}

impl Display for TailwindClear {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindClear {}

impl Display for TailwindIsolation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindIsolation {}

impl Display for TailwindObjectFit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindObjectFit {}

impl Display for TailwindObjectPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindObjectPosition {}

impl Display for TailwindOverflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindOverflow {}

impl Display for TailwindOverscroll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindOverscroll {}

impl Display for TailwindFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindFloat {}

impl Display for TailwindPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindPosition {}

impl Display for TailwindVisibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindVisibility {}

impl Display for TailWindZIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Positive(n) => write!(f, "z-{}", n),
            Self::Negative(n) => write!(f, "-z-{}", n),
            Self::Auto => write!(f, "z-auto"),
        }
    }
}

impl TailwindInstance for TailWindZIndex {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        match self {
            Self::Positive(n) => css_attributes! {
                "z-index" => &n.to_string()
            },
            Self::Negative(n) => css_attributes! {
                "z-index" => &(-(*n as isize)).to_string()
            },
            Self::Auto => css_attributes! {
                "z-index" => "auto"
            },
        }
    }
}
