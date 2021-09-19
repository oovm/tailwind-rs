use crate::modules::borders::*;

///
#[derive(Copy, Clone, Debug)]
pub struct TailwindDivideStyle {
    kind: BorderStyle,
}

impl Display for TailwindDivideStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "divide-{}", self.kind)
    }
}

impl TailwindInstance for TailwindDivideStyle {
    fn inlineable(&self) -> bool {
        false
    }
    fn selectors(&self, _: &TailwindBuilder) -> String {
        // format!(".divide-{} > * + *", self.kind)
        format!(".divide-{}>:not([hidden])~:not([hidden])", self.kind)
    }
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "border-style" => self.kind
        }
    }
}

impl TailwindDivideStyle {
    /// `tracking-solid`
    pub const Solid: Self = Self { kind: BorderStyle::Solid };
    /// `tracking-dashed`
    pub const Dashed: Self = Self { kind: BorderStyle::Dashed };
    /// `tracking-dotted`
    pub const Dotted: Self = Self { kind: BorderStyle::Dotted };
    /// `tracking-double`
    pub const Double: Self = Self { kind: BorderStyle::Double };
    /// `tracking-hidden`
    pub const Hidden: Self = Self { kind: BorderStyle::Hidden };
    /// `tracking-none`
    pub const None: Self = Self { kind: BorderStyle::None };
}
