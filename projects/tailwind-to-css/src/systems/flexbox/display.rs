use super::*;

impl Display for TailwindFlexDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("flex-")?;
        match self {
            Self::Row => f.write_str("row"),
            Self::RowReverse => f.write_str("row-reverse"),
            Self::Column => f.write_str("col"),
            Self::ColumnReverse => f.write_str("col-reverse"),
        }
    }
}

impl TailwindInstance for TailwindFlexDirection {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let direction = match self {
            TailwindFlexDirection::Row => "row",
            TailwindFlexDirection::RowReverse => "row-reverse",
            TailwindFlexDirection::Column => "column-reverse",
            TailwindFlexDirection::ColumnReverse => "column-reverse",
        };
        css_attributes! {
            "flex-direction" => direction
        }
    }
}

impl Display for TailwindFlexWrap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("flex-")?;
        match self {
            Self::Wrap => f.write_str("wrap"),
            Self::WrapReverse => f.write_str("wrap-reverse"),
            Self::NoWrap => f.write_str("nowrap"),
        }
    }
}

impl TailwindInstance for TailwindFlexWrap {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let wrap = match self {
            Self::Wrap => "wrap",
            Self::WrapReverse => "wrap-reverse",
            Self::NoWrap => "nowrap",
        };
        css_attributes! {
            "flex-direction" => wrap
        }
    }
}

impl Display for TailwindFlex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("flex-")?;
        match self {
            Self::None => f.write_str("none"),
            Self::Inherit => {
                todo!()
            }
            Self::Auto { .. } => {
                todo!()
            }
            Self::Percent { .. } => {
                todo!()
            }
        }
    }
}

impl TailwindInstance for TailwindFlex {}

impl Display for TailWindFlexGrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "grow-{}", self.grow)
    }
}

impl TailwindInstance for TailWindFlexGrow {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "flex-grow" => self.grow
        }
    }
}

impl Display for TailWindFlexShrink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "shrink-{}", self.shrink)
    }
}

impl TailwindInstance for TailWindFlexShrink {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "flex-shrink" => self.shrink
        }
    }
}
