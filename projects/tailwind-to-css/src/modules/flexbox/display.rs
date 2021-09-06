use super::*;

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

impl Display for TailwindGridAuto {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindGridAuto {}

impl Display for TailwindGap {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindGap {}

impl Display for TailwindContent {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindContent {}

impl Display for TailwindItems {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindItems {}

impl Display for TailwindSelf {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindSelf {}
