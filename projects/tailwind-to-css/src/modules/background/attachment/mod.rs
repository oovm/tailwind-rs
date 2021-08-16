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
            Self::Scroll => f.write_str("scroll"),
            Self::Fixed => f.write_str("fixed"),
            Self::Local => f.write_str("local"),
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
