use super::*;

impl TailwindInstance for TailwindAspect {
    fn id(&self) -> String {
        format!("break-before: {};", self.kind)
    }
    fn attributes(&self) -> Vec<String> {
        vec![format!("break-before: {};", self.ratio)]
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
    fn attributes(&self) -> Vec<String> {
        match self {
            Self::Before(kind) => vec![format!("break-before: {};", kind)],
            Self::After(kind) => vec![format!("break-after: {};", kind)],
            Self::Inside(kind) => vec![format!("break-inside: {};", kind)],
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
    fn attributes(&self) -> Vec<String> {
        match self {
            Self::Positive(n) => vec![format!("z-index: {};", n)],
            Self::Negative(n) => vec![format!("z-index: -{};", n)],
            Self::Auto => vec![format!("z-index: auto;")],
        }
    }
}
