use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBlend {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindBlend => "mix-blend-mode");

impl Display for TailwindBlend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "mix-blend-{}", self.kind)
    }
}

impl TailwindBlend {
    /// <https://tailwindcss.com/docs/mix-blend-mode>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("mix-blend", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// get class of `<blend-mode>`
    ///
    /// - https://developer.mozilla.org/zh-CN/docs/Web/CSS/blend-mode
    #[inline]
    pub fn get_class(&self) -> String {
        self.kind.to_string()
    }
    /// get properties of `<blend-mode>`
    ///
    /// - https://developer.mozilla.org/zh-CN/docs/Web/CSS/blend-mode
    #[inline]
    pub fn get_properties(&self) -> &str {
        self.kind.get_properties()
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
