use super::*;

#[derive(Copy, Clone, Debug)]
enum Isolation {
    Isolate,
    Auto,
}

///
#[derive(Copy, Clone, Debug)]
pub struct TailwindIsolation {
    kind: Isolation,
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
