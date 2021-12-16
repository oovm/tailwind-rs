use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindAppearance {
    kind: String,
}

impl<T> From<T> for TailwindAppearance
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindAppearance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "appearance-{}", self.kind)
    }
}

impl TailwindInstance for TailwindAppearance {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "appearance" => self.kind
        }
    }
}

impl TailwindAppearance {
    /// https://tailwindcss.com/docs/appearance
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after appearance");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
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
