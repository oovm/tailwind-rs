use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindAppearance {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindAppearance => "appearance");

impl Display for TailwindAppearance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "appearance-{}", self.kind)
    }
}

impl TailwindAppearance {
    /// <https://tailwindcss.com/docs/appearance>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("appearance", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/appearance#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto",
            "button",
            "button-bevel",
            "caret",
            "checkbox",
            "inherit",
            "initial",
            "listbox",
            "media-mute-button",
            "menulist",
            "menulist-button",
            "meter",
            "none",
            "progress-bar",
            "push-button",
            "radio",
            "revert",
            "scrollbarbutton-up",
            "searchfield",
            "slider-horizontal",
            "square-button",
            "textarea",
            "textfield",
            "unset",
        ]);
        set.contains(mode)
    }
}
