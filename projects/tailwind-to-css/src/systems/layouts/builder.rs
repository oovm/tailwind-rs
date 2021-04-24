use super::*;
use std::fmt::{format, Write};

impl TailwindInstance for TailwindAspect {
    fn id(&self) -> String {
        todo!()
    }
    fn attributes(&self) -> Vec<String> {
        vec![format!("break-before: {};", kind)]
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
