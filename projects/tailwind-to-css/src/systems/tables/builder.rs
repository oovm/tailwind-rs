use super::*;

impl TailwindInstance for TailwindBorderCollapse {
    fn id(&self) -> String {
        match self {
            Self::Collapse => "border-collapse",
            Self::Separate => "border-separate",
        }
        .to_string()
    }
    fn attributes(&self) -> BTreeSet<CssAttribute> {
        match self {
            Self::Collapse => css_attributes! {
                "border-collapse" => "collapse"
            },
            Self::Separate => css_attributes! {
                "border-collapse" => "separate"
            },
        }
    }
}

impl TailwindInstance for TailwindTableLayout {
    fn id(&self) -> String {
        match self {
            Self::Auto => "table-auto",
            Self::Fixed => "table-fixed",
        }
        .to_string()
    }
    fn attributes(&self) -> BTreeSet<CssAttribute> {
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
