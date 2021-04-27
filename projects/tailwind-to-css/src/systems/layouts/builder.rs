use super::*;

impl TailwindInstance for TailwindAspect {
    fn id(&self) -> String {
        format!("break-before: {};", self.kind)
    }
    fn attributes(&self) -> Vec<CssAttribute> {
        vec![CssAttribute::new("aspect-ratio", self.ratio)]
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
    fn attributes(&self) -> Vec<CssAttribute> {
        match self {
            Self::Before(kind) => vec![CssAttribute::new("break-before", kind)],
            Self::After(kind) => vec![CssAttribute::new("break-after", kind)],
            Self::Inside(kind) => vec![CssAttribute::new("break-inside", kind)],
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
    fn attributes(&self) -> Vec<CssAttribute> {
        match self {
            Self::Positive(n) => vec![CssAttribute::new("z-index", &n.to_string())],
            Self::Negative(n) => vec![CssAttribute::new("z-index", &n.to_string())],
            Self::Auto => vec![CssAttribute::new("z-index", "auto")],
        }
    }
}
