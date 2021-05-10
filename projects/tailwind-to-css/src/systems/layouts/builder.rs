use super::*;

impl TailwindInstance for TailwindAspect {
    fn id(&self) -> String {
        format!("break-before: {};", self.kind)
    }
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "aspect-ratio" => self.ratio
        }
    }
}

impl TailwindInstance for TailwindBreak {
    fn id(&self) -> String {
        match self {
            Self::Before(kind) => format!("break-before-{}", kind),
            Self::After(kind) => format!("break-after-{}", kind),
            Self::Inside(kind) => format!("break-inside-{}", kind),
        }
    }
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

impl TailwindInstance for TailWindZIndex {
    fn id(&self) -> String {
        match self {
            Self::Positive(n) => format!("z-{}", n),
            Self::Negative(n) => format!("-z-{}", n),
            Self::Auto => format!("z-auto"),
        }
    }
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
