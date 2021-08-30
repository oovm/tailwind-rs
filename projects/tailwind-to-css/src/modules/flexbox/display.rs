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

impl Display for TailWindOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        write!(f, "order-{}", self.order)
    }
}

impl TailwindInstance for TailWindOrder {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let order = match self.negative {
            true => format!("-{}", self.order),
            false => format!("{}", self.order),
        };
        css_attributes! {
            "order" => order
        }
    }
}

impl Display for TailwindGridTemplate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let kind = match self.row {
            true => "rows",
            false => "cols",
        };
        write!(f, "grid-{}-{}", kind, self.unit)
    }
}

impl TailwindInstance for TailwindGridTemplate {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = match self.row {
            true => "grid-template-columns",
            false => "grid-template-columns",
        };
        let grid = match self.unit {
            0 => format!("none"),
            n => format!("repeat({},minmax(0,1fr))", n),
        };
        css_attributes! {
            class => grid
        }
    }
}

impl Display for TailwindColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindColumn {}

impl Display for TailwindRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindRow {}

impl Display for TailwindGridAuto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindGridAuto {}

impl Display for TailwindGap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindGap {}

impl Display for TailwindJustifyContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindJustifyContent {}

impl Display for TailwindJustifyItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindJustifyItems {}

impl Display for TailwindJustifySelf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindJustifySelf {}

impl Display for TailwindContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindContent {}

impl Display for TailwindItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindItems {}

impl Display for TailwindSelf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindSelf {}

impl Display for TailwindPlaceContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindPlaceContent {}

impl Display for TailwindPlaceItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindPlaceItems {}

impl Display for TailwindPlaceSelf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindPlaceSelf {}
