use crate::{css_attributes, CssAttributes, Result, TailwindArbitrary, TailwindBuilder, TailwindInstance};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBlend {
    mode: String,
}

impl Display for TailwindBlend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "mix-blend-{}", self.mode)
    }
}

impl TailwindInstance for TailwindBlend {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "mix-blend-mode" => self.mode
        }
    }
}

impl TailwindBlend {
    /// https://tailwindcss.com/docs/mix-blend-mode
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let mode = input.join("-");
        debug_assert!(Self::check_valid(&mode), "invalid blend mode");
        Ok(Self { mode })
    }
    /// get class of `<blend-mode>`
    ///
    /// - https://developer.mozilla.org/zh-CN/docs/Web/CSS/blend-mode
    #[inline]
    pub fn get_class(&self) -> String {
        self.mode.to_owned()
    }
    /// get properties of `<blend-mode>`
    ///
    /// - https://developer.mozilla.org/zh-CN/docs/Web/CSS/blend-mode
    #[inline]
    pub fn get_properties(&self) -> String {
        self.mode.to_owned()
    }
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "normal",
            "multiply",
            "screen",
            "overlay",
            "darken",
            "lighten",
            "color-dodge",
            "color-burn",
            "hard-light",
            "soft-light",
            "difference",
            "exclusion",
            "hue",
            "saturation",
            "color",
            "luminosity",
        ]);
        set.contains(mode)
    }
}
