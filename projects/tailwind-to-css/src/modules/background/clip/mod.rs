use crate::CssAttribute;

// Background Clip
// Utilities for controlling the bounding box of an element's background.
//
// ​
// Quick reference
// Class
// Properties
// bg-clip-border	background-clip: border-box;
// bg-clip-padding	background-clip: padding-box;
// bg-clip-content	background-clip: content-box;
// bg-clip-text	background-clip: text;
#[derive(Copy, Clone, Debug)]
enum BackgroundClip {
    Border,
    Padding,
    Content,
    Text,
    Global(CssAttribute),
}

// https://tailwindcss.com/docs/background-clip
#[derive(Copy, Clone, Debug)]
pub struct TailwindBackgroundClip {
    kind: BackgroundClip,
}

impl TailwindBackgroundClip {
    /// `bg-clip-border`
    pub const Border: Self = Self { kind: BackgroundClip::Border };
    /// `bg-clip-padding`
    pub const Padding: Self = Self { kind: BackgroundClip::Padding };
    /// `bg-clip-content`
    pub const Content: Self = Self { kind: BackgroundClip::Content };
    /// `bg-clip-text`
    pub const Text: Self = Self { kind: BackgroundClip::Text };
}
