use super::*;

impl Display for TailwindAspect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => {
                write!(f, "aspect-auto")
            }
            Self::Arbitrary(a, b) => {
                write!(f, "aspect-{}/{}", a, b)
            }
        }
    }
}

impl TailwindInstance for TailwindAspect {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let ar = match self {
            Self::Auto => "auto",
            Self::Arbitrary(_, _) => {
                todo!()
            }
        };
        css_attributes! {
            "aspect-ratio" => ar
        }
    }
}

impl Display for TailwindContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindContainer {}

impl Display for TailwindBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Before(kind) => write!(f, "break-before-{}", kind),
            Self::After(kind) => write!(f, "break-after-{}", kind),
            Self::Inside(kind) => write!(f, "break-inside-{}", kind),
        }
    }
}

impl TailwindInstance for TailwindBreak {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        match self {
            Self::Before(kind) => css_attributes! {
                "break-before" => kind
            },
            Self::After(kind) => css_attributes! {
                "break-after" => kind
            },
            Self::Inside(kind) => css_attributes! {
                "break-inside" => kind
            },
        }
    }
}

impl Display for TailwindColumns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindColumns {
    fn id(&self) -> String {
        todo!()
    }
}

impl Display for TailwindBoxDecorationBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindBoxDecorationBreak {
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
