use super::*;

impl Display for TailwindBorderCollapse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Collapse => "border-collapse",
            Self::Separate => "border-separate",
        };
        f.write_str(text)
    }
}

impl TailwindInstance for TailwindBorderCollapse {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
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
