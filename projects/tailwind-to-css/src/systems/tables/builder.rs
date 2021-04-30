use super::*;

impl TailwindInstance for TailwindBorderCollapse {
    fn id(&self) -> String {
        match self {
            Self::Collapse => {}
            Self::Separate => {}
        }
    }
    fn attributes(&self) -> BTreeSet<CssAttribute> {
        match self {
            Self::Collapse => css_attributes! {}
            Self::Separate => css_attributes! {}
        }
    }
}

impl TailwindInstance for TailwindTableLayout {
    fn id(&self) -> String {
        match self {
            Self::Auto => {}
            Self::Fixed => {}
        }
    }
    fn attributes(&self) -> BTreeSet<CssAttribute> {
        match self {
            Self::Auto => css_attributes! {}
            Self::Fixed => css_attributes! {}
        }
    }
}
