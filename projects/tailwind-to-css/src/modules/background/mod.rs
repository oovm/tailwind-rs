use super::*;
mod builder;
mod display;
#[cfg(test)]
mod test;

/// https://developer.mozilla.org/zh-CN/docs/Web/CSS/background-attachment
#[derive(Copy, Clone, Debug)]
enum AttachmentKind {
    Scroll,
    Fixed,
    Local,
    Global(CssBehavior),
}

#[doc = include_str ! ("aspect-ratio.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindBackgroundAttachment {
    kind: AttachmentKind,
}

#[doc = include_str ! ("aspect-ratio.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundColor {
    pub(crate) color: ColorResolver,
}

#[doc = include_str ! ("aspect-ratio.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundBrightness {
    brightness: TailwindBrightness,
}
