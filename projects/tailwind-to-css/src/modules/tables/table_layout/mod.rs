use super::*;

#[doc = include_str ! ("readme.md")]
#[derive(Copy, Clone, Debug)]
pub enum TailwindTableLayout {
    Auto,
    Fixed,
}

impl Display for TailwindTableLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Auto => "table-auto",
            Self::Fixed => "table-fixed",
        };
        f.write_str(text)
    }
}

impl TailwindInstance for TailwindTableLayout {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        match self {
            Self::Auto => css_attributes! {
                "table-layout" => "auto"
            },
            Self::Fixed => css_attributes! {
                "table-layout" => "fixed"
            },
        }
    }
}
