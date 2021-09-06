use super::*;

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindBackgroundAttachment {
    kind: AttachmentKind,
}

#[derive(Copy, Clone, Debug)]
enum AttachmentKind {
    Scroll,
    Fixed,
    Local,
    // Global(CssBehavior),
}

impl Display for AttachmentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scroll => write!(f, "scroll"),
            Self::Fixed => write!(f, "fixed"),
            Self::Local => write!(f, "local"),
        }
    }
}

impl Display for TailwindBackgroundAttachment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBackgroundAttachment {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "background-attachment" => self.kind
        }
    }
}

impl TailwindBackgroundAttachment {
    /// `bg-fixed`
    pub const Fixed: Self = Self { kind: AttachmentKind::Fixed };
    /// `bg-local`
    pub const Local: Self = Self { kind: AttachmentKind::Local };
    /// `bg-scroll`
    pub const Scroll: Self = Self { kind: AttachmentKind::Scroll };
}
